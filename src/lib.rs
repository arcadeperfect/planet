#![allow(dead_code)]
use anyhow::Result;
use bitmap::{apply_blur, get_initial_planet_map, umap_to_image_buffer};
use cellular_automata::simulate;
use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgba};
use marching_squares::march_squares_rgba;
use noise::permutationtable::PermutationTable;

use std::time::Instant;
pub use types::PlanetOptions;
use types::{FMap, FractalNoiseOptions, PlanetData, PlanetMap};


mod noise_example;
mod bitmap;
mod cellular_automata;
mod marching_squares;
mod traits;
mod utils;
mod noise_circle;
mod room;
pub mod types;
// pub struct PlumbetPlugin;

// #[derive(Clone, Debug)]
// pub struct PlanetData {
//     pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
//     pub planet_map: PlanetMap,
//     pub poly_lines: Vec<Vec<Vec2>>,
// }

// impl PlanetData {
//     /// return the poly lines as a flattened list where each pair represents a line segment
//     /// this results in a lot of doubled points, but this is how the shader likes it
//     pub fn get_line_list(&self) -> Vec<Vec3> {
//         flatten_and_zip(&self.poly_lines)
//     }
// }

pub struct PlanetBuilder {
    hasher: PermutationTable,
}

impl PlanetBuilder {
    pub fn new(seed: u32) -> Self {
        PlanetBuilder {
            hasher: PermutationTable::new(seed),
        }
    }

    pub fn build(&self, options: PlanetOptions, fractal_options: Vec<&FractalNoiseOptions>) -> Result<PlanetData> {
        // let start = Instant::now();

        let (map, altitude_field, depth_field) = get_initial_planet_map(&options, fractal_options, &self.hasher)?;

        // let duration1 = start.elapsed();
        // let start = Instant::now();

        let rooms = simulate(&options, &map, &depth_field);

        // let duration2 = start.elapsed();
        // let start = Instant::now();

        let map = sub(&rooms, &map, &depth_field, 1.-options.crust_thickness);

        // let duration3 = start.elapsed();
        // let start = Instant::now();

        let mut image = umap_to_image_buffer(&map);

        // let duration4 = start.elapsed();
        // let start = Instant::now();

        image = apply_blur(&image, options.blur);

        // let duration5 = start.elapsed();
        // let start = Instant::now();

        let c = march_squares_rgba(&image)?;

        // let duration6 = start.elapsed();

        /*         println!("");
        println!("initial: \t{}ms", duration1.as_millis());
        println!("rooms: \t\t{}ms", duration2.as_millis());
        println!("sub: \t\t{}ms", duration3.as_millis());
        println!("umap_to_rgba: \t {}ms", duration4.as_millis());
        println!("blur: \t\t {}ms", duration5.as_millis());
        println!("march: \t\t {}ms", duration6.as_millis()); */

        let mut planet_map: PlanetMap = PlanetMap::empty(options.resolution as usize);
        planet_map.main = Some(map);
        planet_map.altitude = Some(altitude_field);
        planet_map.depth = Some(depth_field);
        planet_map.rooms = Some(rooms);

        Ok(PlanetData {
            image: Some(image),
            planet_map,
            poly_lines: c,
        })
    }
}

fn flatten_and_zip(vertices: &Vec<Vec<Vec2>>) -> Vec<Vec3> {
    vertices
        .iter()
        .flat_map(|digit_points| digit_points.windows(2).flat_map(|window| window))
        .map(|v| Vec3::new(v.x, v.y, 0.0))
        .collect()
}

fn sub(this: &Vec<Vec<u8>>, from: &Vec<Vec<u8>>, mask: &FMap, thresh: f32) -> Vec<Vec<u8>> {
    from.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| {
                    
                    if mask[y][x] > thresh {
                        return val
                    }

                    let inverted = if this[y][x] == 1 { 0 } else { 1 };
                    val * inverted
                })
                .collect()
        })
        .collect()
}

// pub type UMap8 = Vec<Vec<u8>>;
// pub type UMap16 = Vec<Vec<u16>>;

// trait Subtract<Rhs = Self, Output = Self> {
//     fn subtract(&self, other: &Rhs) -> Output;
// }

// impl Subtract for UMap16 {
//     fn subtract(&self, other: &Self) -> Self {
//         self.iter()
//             .zip(other.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(s, o)| (s - o).max(0))
//                     .collect()
//             })
//             .collect()
//     }
// }

// impl Subtract<UMap16, UMap8> for UMap8 {
//     fn subtract(&self, other: &UMap16) -> UMap8 {
//         self.iter()
//             .zip(other.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(s, o)| ((*s as u16) - o).max(0) as u8)
//                     .collect()
//             })
//             .collect()
//     }
// }

// impl Subtract<UMap8, UMap16> for UMap16 {
//     fn subtract(&self, other: &UMap8) -> UMap16 {
//         self.iter()
//             .zip(other.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(s, o)| (s - (*o as u16)).max(0))
//                     .collect()
//             })
//             .collect()
//     }
// }
