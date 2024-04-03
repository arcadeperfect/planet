use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgba};
use anyhow::{anyhow, Result};

use crate::{roooms::Roooms, tile_map::TileMap, types::{Coord, PlanetMap}};

pub use crate::marching_squares::march_squares_rgba;


#[derive(Clone, Debug)]
pub struct PlanetData {
    pub image:ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub planet_map: PlanetMap,
    // pub polylines: Vec<Vec<Vec2>>,
    pub tile_map: TileMap,
    pub mst: Option<Vec<(Coord, Coord)>>,
    pub roooms: Option<Roooms>,
}

impl PlanetData {
    /// return the poly lines as a flattened list where each pair represents a line segment
    // pub fn get_line_list(&self) -> Result<Vec<Vec3>> {
    //     let polylines = march_squares_rgba(&self.image)?;
    //     Ok(flatten_and_zip(&polylines))
    // }

    // pub fn get_polylines(&self) -> Result<Vec<Vec<Vec2>>> {
    //     let polylines = march_squares_rgba(&self.image)?;
    //     Ok(polylines)
    // }

    pub fn get_dimension(&self) -> usize {

            self.planet_map.resolution            
        }
    }


pub fn flatten_and_zip(vertices: &Vec<Vec<Vec2>>) -> Vec<Vec3> {
    vertices
        .iter()
        .flat_map(|digit_points| digit_points.windows(2).flat_map(|window| window))
        .map(|v| Vec3::new(v.x, v.y, 0.0))
        .collect()
}


