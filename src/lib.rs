#![allow(dead_code)]

use crate::{
    bit_map::{nearest_neighbor, simple_line, thick_line},
    room::generate_rooms,
    roooms::Roooms,
    tile_map::{FromUMap, Tile, TileMap},
    triangulation::{triangulation_to_coords, RoomTriangulation},
    types::Coord,
};
use anyhow::Result;
use bit_map::{apply_blur, get_initial_planet_map, umap_to_image_buffer};
use cellular_automata::simulate_ca;
use marching_squares::march_squares_rgba;
use noise::permutationtable::PermutationTable;
use planet_data::PlanetData;
pub use types::PlanetOptions;
use types::{FMap, FractalNoiseOptions, PlanetMap, UMap8};

mod bit_map;
mod cellular_automata;
mod marching_squares;
mod noise_circle;
mod noise_example;
pub mod planet_data;
pub mod room;
pub mod roooms;
pub mod tile_map;
mod traits;
pub mod triangulation;
pub mod types;
mod utils;

pub struct PlanetBuilder {
    hasher: PermutationTable,
}

impl PlanetBuilder {
    pub fn new(seed: u32) -> Self {
        PlanetBuilder {
            hasher: PermutationTable::new(seed),
        }
    }

    pub fn build(
        &self,
        options: PlanetOptions,
        fractal_options: Vec<&FractalNoiseOptions>,
    ) -> Result<PlanetData> {
        tracing::info!("##### new planet #####");

        let (mut initial_planet_map, altitude_field, depth_field) =
            get_initial_planet_map(&options, fractal_options)?;

        let mut cave_map_raw: UMap8 = simulate_ca(&options, &initial_planet_map, &depth_field);

        let mut tile_map = TileMap::rooms_planet_combiner(&initial_planet_map, &cave_map_raw);

        // let mut tilemap_copy = tile_map.clone();

        // let rooms = generate_rooms(&mut tile_map);

        // let room_container = RoomTriangulation::new(rooms);

        let roooms = Roooms::new(&mut tile_map).ok();

        // let triangulation = triangulation::delaunate_rooms(&roooms.rooms)
        //     .ok()
        //     .map(|triangulation| Some(triangulation))
        //     .unwrap_or(None);

        // let mst = triangulation.as_ref().map(|tr| _mst_to_coords(tr, &rooms));
        if let Some(rms) = &roooms {
            
                 
                for (x, (a, b)) in rms.get_mst_as_coord().iter().enumerate() {
                    
                    let l = thick_line(a, b, 3);
                    for p in l {
                        match tile_map[p.x][p.y] {
                            Tile::Wall => {
                                tile_map[p.x][p.y] = Tile::Tunnel(x as u16);
                                initial_planet_map[p.x][p.y] = 0;
                            }
                            _ => {}
                        }
                        // tile_map[p.x][p.y] = Tile::Space;
                    }
                }
            
        }

        let map_main: UMap8 = thresh_sub(
            &cave_map_raw,
            &initial_planet_map,
            &depth_field,
            1. - options.crust_thickness, //todo don't do thickness like this, do it before rooms are calculated
        );

        let mut image = umap_to_image_buffer(&map_main);

        image = apply_blur(&image, options.blur);

        let polylines = march_squares_rgba(&image)?;

        let mut maps: PlanetMap = PlanetMap::empty(options.resolution as usize);
        maps.main = Some(map_main);
        maps.altitude = Some(altitude_field);
        maps.depth = Some(depth_field);
        maps.rooms_raw = Some(cave_map_raw);

        Ok(PlanetData {
            image: Some(image),
            planet_map: maps,
            poly_lines: polylines,
            tile_map: Some(tile_map),
            // rooms: Some(rooms),
            // triangulation: triangulation,
            mst: None,
            roooms,
        })
    }
}

fn thresh_sub(this: &Vec<Vec<u8>>, from: &Vec<Vec<u8>>, mask: &FMap, thresh: f32) -> Vec<Vec<u8>> {
    from.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| {
                    if mask[y][x] > thresh {
                        return val;
                    }

                    let inverted = if this[y][x] == 1 { 0 } else { 1 };
                    val * inverted
                })
                .collect()
        })
        .collect()
}

fn mult(this: &Vec<Vec<u8>>, from: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    from.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| val * this[y][x])
                .collect()
        })
        .collect()
}
