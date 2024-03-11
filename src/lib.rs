use anyhow::Result;
use bitmap::{apply_blur, get_initial_planet_map, umap_to_rgba};
use cellular_automata::simulate;
use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgba};
use marching_squares::{march_squares_rgba, march_squares_umap};
use noise::permutationtable::PermutationTable;
use rayon::iter::Map;
pub use types::PlanetOptions;
use types::{PlanetMap, UMap16, UMap8};

pub struct PlumbetPlugin;

mod bitmap;
mod cellular_automata;
mod marching_squares;
mod traits;
mod types;
mod utils;

#[derive(Clone, Debug)]
pub struct PlanetData {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>, 
    pub planetMap: PlanetMap,
    pub poly_lines: Vec<Vec<Vec2>>,
}

impl PlanetData {
    /// return the poly lines as a flattened list where each pair represents a line segment
    /// this results in a lot of doubled points, but this is how the shader likes it
    pub fn get_line_list(&self) -> Vec<Vec3> {
        flatten_and_zip(&self.poly_lines)
    }
}

pub struct PlanetBuilder {
    hasher: PermutationTable,
}

impl PlanetBuilder {
    pub fn new(seed: u32) -> Self {
        PlanetBuilder {
            hasher: PermutationTable::new(seed),
        }
    }

    pub fn build(&self, options: PlanetOptions) -> Result<PlanetData> {
        
        
        let (mut map, altitude_field) = get_initial_planet_map(&options, &self.hasher)?;
        let rooms = simulate(&options, &map, &altitude_field);

        let out = sub(rooms, map);
             
        // let out = apply_blur(out, 5.);

        // println!("{:?}", out);

        let mut image = umap_to_rgba(&out);

        

        image = apply_blur(&image, options.blur);
        let c = march_squares_rgba(&image)?;
        // let contours = march_squares_umap(&out)?;

        let mut planetMap: PlanetMap = PlanetMap::empty(options.resolution as usize);
        planetMap.main = Some(out);
        planetMap.altitude = Some(altitude_field);

        Ok(PlanetData {
            image: image,
            planetMap,
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

fn sub(this: Vec<Vec<u8>>, from: Vec<Vec<u8>> ) -> Vec<Vec<u8>> {
    from.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, &val)| {
            // Invert the second matrix's value (1 becomes 0, 0 becomes 1)
            let inverted = if this[y][x] == 1 { 0 } else { 1 };
            // Multiply the first matrix's value by the inverted value
            val * inverted
        }).collect()
    }).collect()
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