use crate::{tile_map::TileMap, types::{FMap, FractalNoiseOptions, UMap8}, PlanetOptions};
use anyhow::Result;

#[derive(Default)]
pub struct MapData{
    pub raw_map: UMap8,
    pub altitude_field: FMap,
    pub depth_field: FMap,
    pub surface_distance_field: FMap,
}

// pub tile_map: TileMap

impl MapData {

    

    // // pub fn new()


    // pub fn initialize(
    //     options: &PlanetOptions,
    //     fractal_options: Vec<&FractalNoiseOptions>,
    // ) -> Result<MapData>
    // {
        
    // }
    
}