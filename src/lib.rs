#![allow(dead_code)]

use std::usize;

use crate::{
    bit_map::{
        image_buffer_to_fmap, image_buffer_to_umap,
        umap_to_fmap, MapOpps,
    },
    cellular_automata::simulate_ca,
    debug_print::MapDebug,
    map_data::MapData,
    noise_circle::simple_circle,
    room::closest_tiles,
    roooms::Roooms,
    tile_map::{FromUMap, Tile, TileMap},
    utils::random_distribution_mask_weighted,
};
use anyhow::{anyhow, Result};
use bit_map::{
    get_initial_planet_map, noise_line, rgba_image_blur,
    umap_to_image_buffer,
};
use image::RgbaImage;

use marching_squares::march_squares_rgba;
use noise::permutationtable::PermutationTable;
use planet_data::PlanetData;
use room::Room;
pub use types::PlanetOptions;
use types::{
    Blank, Coord, FMap, FractalNoiseOptions, PlanetMap,
    PolyLines, UMap8,
};

mod bit_map;
mod cellular_automata;
mod debug_print;
mod map_data;
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

        let r = options.resolution();
        let mut md = MapData::default();
        let v = get_initial_planet_map(
            &options,
            fractal_options,
        )?;
        md.raw_map = v.0;
        md.altitude_field = v.1;
        md.depth_field = v.2;

        let mut map_main = UMap8::blank(r as usize);
        let mut tile_map = TileMap::blank(r as usize);
        let mut mask: Option<FMap> = None;
        let blurred_edge = rgba_image_blur(
            &umap_to_image_buffer(&md.raw_map),
            options.ca_options.mask_options.mult,
        );
        let mut msk = image_buffer_to_fmap(&blurred_edge);

        msk.clamp(0., 1.);
        msk.invert();
        msk.mult(2.);
        msk.clamp(0., 1.);

        let init_state = random_distribution_mask_weighted(
            options.resolution(),
            options.ca_options.init_weight,
            &msk,
            true,
            options.ca_options.seed,
        );

        mask = Some(msk);

        match &options.rooms {
            true => {
                let mut cave_map_raw: UMap8 =
                    simulate_ca(&options, init_state, &md);
                let center_hole =
                    simple_circle(options.radius * 10., r);
                for y in 0..r {
                    for x in 0..r {
                        if center_hole[x as usize]
                            [y as usize]
                            > 0
                        {
                            cave_map_raw[x as usize]
                                [y as usize] = 1;
                        }
                    }
                }

                tile_map = TileMap::from_planet_and_caves(
                    &md.raw_map,
                    &cave_map_raw,
                );
                let roooms =
                    Roooms::new(&mut tile_map).ok();

                if let Some(roooms) = &roooms {
                    if let Some(mst) = roooms.mst.as_ref() {
                        if options.tunnels {
                            match connect_rooms(
                                &roooms.rooms,
                                mst,
                                &mut tile_map,
                                &mut md.raw_map,
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    tracing::error!(
                                        "{}", e
                                    );
                                }
                            }
                        }
                    }
                }

                map_main = thresh_sub(
                    &cave_map_raw,
                    &md.raw_map,
                    &md.depth_field,
                    1. - options.crust_thickness, //todo don't do thickness like this, do it before rooms are calculated
                );

                let mut image =
                    umap_to_image_buffer(&map_main);

                image =
                    rgba_image_blur(&image, options.blur);

                // let polylines = march_squares_rgba(&image)?;

                let maps: PlanetMap = PlanetMap {
                    resolution: r as usize,
                    main: map_main,
                    rooms_raw: None,
                    edges: None,
                    altitude: md.altitude_field,
                    depth: md.depth_field,
                    edge_distance_field: None,
                    mask,
                };

                return Ok(PlanetData {
                    planet_map: maps,
                    image,
                    // polylines,
                    tile_map,
                    mst: None,
                    roooms: roooms,
                });
            }
            false => {
                tile_map = TileMap::from_u_map(&md.raw_map);
                map_main = md.raw_map;
            }
        }

        let mut image = umap_to_image_buffer(&map_main);

        image = rgba_image_blur(&image, options.blur);

        // let polylines = march_squares_rgba(&image)?;

        let maps: PlanetMap = PlanetMap {
            resolution: r as usize,
            main: map_main,
            rooms_raw: None,
            edges: None,
            altitude: md.altitude_field,
            depth: md.depth_field,
            edge_distance_field: None,
            mask,
        };

        Ok(PlanetData {
            planet_map: maps,
            image,
            // polylines,
            tile_map,
            mst: None,
            roooms: None,
        })
    }
}

// fn get_edge_distance_field(map: &UMap8) -> FMap {
//     let mut out = FMap::blank(map.len());

// }

// pub fn remarch(rgba: &RgbaImage) -> Result<PolyLines> {
//     march_squares_rgba(rgba)
// }

// pub trait MapOpps {
//     fn mult(&mut self, mult: f32);
//     fn lift(&mut self, lift: f32);
//     fn invert(&mut self);
//     fn clamp(&mut self, min: f32, max: f32);
//     fn remap(&mut self, low1: f32, high1: f32, low2: f32, high2: f32);
// }

// impl MapOpps for FMap {
//     fn mult(&mut self, mult: f32) {
//         for row in self.iter_mut() {
//             for value in row.iter_mut() {
//                 *value *= mult;
//             }
//         }
//     }
//     fn lift(&mut self, lift: f32) {
//         for row in self.iter_mut() {
//             for value in row.iter_mut() {
//                 *value = 1. - ((1. - *value) * lift)
//             }
//         }
//     }
//     fn invert(&mut self) {
//         for row in self.iter_mut() {
//             for value in row.iter_mut() {
//                 *value = 1. - *value;
//             }
//         }
//     }
//     fn clamp(&mut self, min: f32, max: f32) {
//         for row in self.iter_mut() {
//             for value in row.iter_mut() {
//                 *value = value.clamp(min, max)
//             }
//         }
//     }

//     fn remap(&mut self, low1: f32, high1: f32, low2: f32, high2: f32) {
//         for row in self.iter_mut() {
//             for value in row.iter_mut() {
//                 *value = (*value - low1) / (high1 - low1) * (high2 - low2) + low2
//             }
//         }
//     }
// }

fn thresh_sub(
    this: &UMap8,
    from: &UMap8,
    mask: &FMap,
    thresh: f32,
) -> UMap8 {
    from.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| {
                    if mask[y][x] > thresh {
                        return val;
                    }

                    let inverted =
                        if this[y][x] == 1 { 0 } else { 1 };
                    val * inverted
                })
                .collect()
        })
        .collect()
}

fn mult(
    this: &Vec<Vec<u8>>,
    from: &Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {
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

fn connect_rooms(
    rooms: &Vec<Room>,
    mst: &Vec<(usize, usize)>,
    tile_map: &mut TileMap,
    umap: &mut UMap8,
) -> Result<()> {
    let max_p = tile_map.len();

    let tunnel_thickness = umap.len() as f32 * 0.008;

    for index_pair in mst {
        line_between_rooms(
            &rooms[index_pair.0],
            &rooms[index_pair.1],
            tunnel_thickness as u32,
        )
        .iter()
        .try_for_each(|p| {
            if p.x > max_p - 1 || p.y > max_p - 1 {
                Err(anyhow!("p.x > max_p || p.y > max_p"))
            } else {
                match tile_map[p.x][p.y] {
                    Tile::Wall => {
                        tile_map[p.x][p.y] =
                            Tile::Tunnel(0);
                        umap[p.x][p.y] = 0;
                    }
                    _ => {}
                }
                Ok(())
            }
        })?;
    }

    Ok(())
}

fn line_between_rooms(
    a: &Room,
    b: &Room,
    thickness: u32,
) -> Vec<Coord> {
    let c = closest_tiles(a, b);
    // thick_line(&c.0, &c.1, 3)
    // variable_line(&c.0, &c.1, 3, 3.)
    noise_line(&c.0, &c.1, 3, 5., 0.03)
}

// fn get_surface(map: UMap8) -> Vec<Coord> {
//     let mut out = Vec::new();

//     for x in 0..map.len() {
//         for y in 0..map.len() {
//             let v = map[x][y];
//             if check_neighbors_horizonatl_or_vertical(x, y, &map) {
//                 out.push(Coord { x, y });
//             }
//         }
//     }
//     out
// }

// fn check_neighbors_horizonatl_or_vertical(x: usize, y: usize, map: &UMap8) -> bool {
//     if map[x][y] == 0 {
//         return false;
//     }

//     // left
//     if x > 0 {
//         if map[x - 1][y] == 0 {
//             return true;
//         }
//     }

//     // right
//     if x < map.len() - 1 {
//         if map[x + 1][y] == 0 {
//             return true;
//         }
//     }

//     // above
//     if y > 0 {
//         if map[x][y - 1] == 0 {
//             return true;
//         }
//     }

//     // below

//     if y < map.len() - 1 {
//         if map[x][y + 1] == 0 {
//             return true;
//         }
//     }

//     false
// }
