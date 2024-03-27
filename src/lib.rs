#![allow(dead_code)]

use crate::{
    room::process_rooms,
    tile_map::{FromUMap, TileMap},
    triangulation::mst_to_coords,
    types::Coord,
};
use anyhow::Result;
use bitmap::{apply_blur, get_initial_planet_map, umap_to_image_buffer};
use cellular_automata::simulate;
use delaunator::Triangulation;
use marching_squares::march_squares_rgba;
use noise::permutationtable::PermutationTable;
use planet_data::PlanetData;
pub use types::PlanetOptions;
use types::{FMap, FractalNoiseOptions, PlanetMap, UMap8};

mod bitmap;
mod cellular_automata;
mod marching_squares;
mod noise_circle;
mod noise_example;
pub mod planet_data;
pub mod room;
pub mod tile_map;
mod traits;
mod triangulation;
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

        let (initial_planet_map, altitude_field, depth_field) =
            get_initial_planet_map(&options, fractal_options)?;

        let room_map_raw: UMap8 = simulate(&options, &initial_planet_map, &depth_field);

        let mut tile_map = TileMap::rooms_planet_combiner(&initial_planet_map, &room_map_raw);

        let room_structs = process_rooms(&mut tile_map);

        let triangulation: Option<Triangulation> = triangulation::delaunate_rooms(&room_structs)
            .ok()
            .map(|triangulation| Some(triangulation))
            .unwrap_or(None);

        let mst = triangulation
            .as_ref()
            .map(|tr| mst_to_coords(tr, &room_structs));

        let map_main: UMap8 = sub(
            &room_map_raw,
            &initial_planet_map,
            &depth_field,
            1. - options.crust_thickness,
        );

        // map_main.debug_print_pretty();

        let mut image = umap_to_image_buffer(&map_main);

        image = apply_blur(&image, options.blur);

        let polylines = march_squares_rgba(&image)?;

        let mut maps: PlanetMap = PlanetMap::empty(options.resolution as usize);
        maps.main = Some(map_main);
        maps.altitude = Some(altitude_field);
        maps.depth = Some(depth_field);
        maps.rooms_raw = Some(room_map_raw);

        Ok(PlanetData {
            image: Some(image),
            planet_map: maps,
            poly_lines: polylines,
            tile_map: Some(tile_map),
            rooms: Some(room_structs),
            triangulation: triangulation,
            mst,
        })
    }
}

fn sub(this: &Vec<Vec<u8>>, from: &Vec<Vec<u8>>, mask: &FMap, thresh: f32) -> Vec<Vec<u8>> {
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
