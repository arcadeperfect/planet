use anyhow::Result;
use bitmap::get_initial_planet_map;
use glam::{Vec2, Vec3};
use marching_squares::march_squares;
use noise::permutationtable::PermutationTable;
use types::PlanetMap;
pub use types::PlanetOptions;

pub struct PlumbetPlugin;

mod bitmap;
mod cellular_automata;
mod marching_squares;
mod traits;
mod types;
mod utils;

#[derive(Clone, Debug)]
pub struct PlanetData {
    pub map: PlanetMap,
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



        let contours = march_squares(&map)?;


        let mut planetMap: PlanetMap = PlanetMap::new(options.resolution as usize);
        planetMap.planet = map;

        Ok(PlanetData {
            map: planetMap,
            poly_lines: contours,
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
