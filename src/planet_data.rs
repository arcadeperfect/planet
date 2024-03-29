use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgba};

use crate::{
    roooms::Roooms,
    tile_map::TileMap,
    types::{Coord, PlanetMap},
};

#[derive(Clone, Debug)]
pub struct PlanetData {
    pub image:ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub planet_map: PlanetMap,
    pub polylines: Vec<Vec<Vec2>>,
    pub tile_map: TileMap,
    pub mst: Option<Vec<(Coord, Coord)>>,
    pub roooms: Option<Roooms>,
}

impl PlanetData {
    /// return the poly lines as a flattened list where each pair represents a line segment
    pub fn get_line_list(&self) -> Vec<Vec3> {
        flatten_and_zip(&self.polylines)
    }

    pub fn get_dimension(&self) -> usize {

            self.planet_map.resolution            
        }
    }


fn flatten_and_zip(vertices: &Vec<Vec<Vec2>>) -> Vec<Vec3> {
    vertices
        .iter()
        .flat_map(|digit_points| digit_points.windows(2).flat_map(|window| window))
        .map(|v| Vec3::new(v.x, v.y, 0.0))
        .collect()
}
