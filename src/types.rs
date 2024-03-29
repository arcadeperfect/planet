use crate::room::Room;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::ops::Add;

pub type PolyLine = Vec<Vec2>;
pub type PolyLines = Vec<PolyLine>;
pub type UMap8 = Vec<Vec<u8>>;
pub type UMap16 = Vec<Vec<u16>>;
pub type IMap16 = Vec<Vec<i16>>;
pub type FMap = Vec<Vec<f32>>;

pub trait Blank {
    fn blank(resolution: usize) -> Self;
}

impl Blank for UMap8 {
    fn blank(resolution: usize) -> Self {
        vec![vec![0; resolution]; resolution]
    }
}

impl Blank for UMap16 {
    fn blank(resolution: usize) -> Self {
        vec![vec![0; resolution]; resolution]
    }
}

impl Blank for FMap {
    fn blank(resolution: usize) -> Self {
        vec![vec![0.; resolution]; resolution]
    }
}

#[derive(Clone, Debug)]
pub struct PlanetMap {
    pub resolution: usize,
    pub main: UMap8,
    pub rooms_raw: Option<UMap8>,
    pub edges: Option<UMap8>,
    pub altitude: FMap,
    pub depth: FMap,
    pub edge_distance_field: Option<FMap>,
    pub mask: Option<FMap>,
}

impl PlanetMap {
    pub fn blank(resolution: usize) -> Self {
        PlanetMap {
            resolution,
            main: vec![vec![0; resolution]; resolution],
            rooms_raw: Some(vec![vec![0; resolution]; resolution]),
            edges: Some(vec![vec![0; resolution]; resolution]),
            altitude: vec![vec![0.; resolution]; resolution],
            depth: vec![vec![0.; resolution]; resolution],
            edge_distance_field: Some(vec![vec![0.; resolution]; resolution]),
            mask: Some(vec![vec![0.; resolution]; resolution]),
        }
    }

    // pub fn empty(resolution: usize) -> Self {
    //     PlanetMap {
    //         resolution,
    //         main: None,
    //         rooms_raw: None,
    //         edges: None,
    //         altitude: None,
    //         depth: None,
    //         edge_distance_field: None,
    //         mask: None,
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct PlanetOptions {
    pub seed: u32,
    pub radius: f32,
    pub resolution: u32,
    pub weight: f32,
    pub thresh: u32,
    pub blur: f32,
    pub min_room_size: usize,
    pub crust_thickness: f32,
    pub ca_iterations: u32,
    pub ca_search_radius: u32,
    pub ca_misc: i32,
    pub invert_ca: bool,
    pub mask_frequency: f64,
    pub mask_z: f64,
    pub global_amplitude: f32,
    pub displacement_scale: f64,
    pub displacement_frequency: f64,
    pub rooms: bool
}

impl PlanetOptions {
    pub fn new(
        seed: u32,
        radius: f32,
        resolution: u32,
        thresh: u32,
        weight: f32,
        blur: f32,
        crust_thickness: f32,
        min_room_size: usize,
        ca_iterations: u32,
        ca_search_radius: u32,
        ca_misc: i32,
        invert_ca: bool,
        mask_frequency: f64,
        mask_z: f64,
        global_amplitude: f32,
        displacement_scale: f64,
        displacement_frequency: f64,
        rooms: bool
    ) -> Self {
        Self {
            seed,
            radius,
            resolution: resolution.max(8),
            thresh,
            weight,
            blur,
            min_room_size,
            crust_thickness,
            ca_search_radius,
            ca_iterations,
            ca_misc,
            invert_ca,
            mask_frequency,
            mask_z,
            global_amplitude,
            displacement_scale,
            displacement_frequency,
            rooms
        }
    }

    pub fn resolution(&self) -> u32 {
        self.resolution.max(8)
    }
}

impl Default for PlanetOptions {
    fn default() -> Self {
        Self {
            seed: 1,
            radius: 1.,
            resolution: 100,
            weight: 50.,
            thresh: 4,
            blur: 1.,
            min_room_size: 20,
            crust_thickness: 0.,
            ca_iterations: 10,
            ca_search_radius: 10,
            ca_misc: 0,
            invert_ca: false,
            mask_frequency: 0.5,
            mask_z: 0.0,
            global_amplitude: 1.0,
            displacement_scale: 1.0,
            displacement_frequency: 1.0,
            rooms: false
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FractalNoiseOptions {
    pub frequency: f64,
    pub lacunarity: f64,
    pub octaves: usize,
    pub persistence: f64,
    pub amplitude: f32,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    pub fn default() -> Coord {
        Coord { x: 0, y: 0 }
    }

    pub fn into_world_normalized_vec2(self, r: &u32) -> Vec2 {
        Vec2::new(
            self.x as f32 / *r as f32 * 2.0 - 1.0,
            -(self.y as f32 / *r as f32 * 2.0 - 1.0),
        )
    }

    pub fn into_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn max() -> Coord {
        Coord {
            x: usize::MAX,
            y: usize::MAX,
        }
    }

    pub fn min() -> Coord {
        Coord { x: 0, y: 0 }
    }
}

impl Add<(usize, usize)> for Coord {
    type Output = Coord;
    fn add(self, other: (usize, usize)) -> Self::Output {
        Coord {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Coord;
    fn add(self, other: (i32, i32)) -> Self::Output {
        let mut new_x: i32 = self.x as i32 + other.0;
        let mut new_y: i32 = self.y as i32 + other.1;

        new_x = new_x.max(0);
        new_y = new_y.max(0);

        Coord {
            x: new_x as usize,
            y: new_y as usize,
        }
    }
}

pub trait DebugPrint {
    fn debug_print(&self);
}

impl DebugPrint for Vec<Room> {
    fn debug_print(&self) {
        println!("");
        println!("Debug print room vec --- {:?} rooms \n", self.len());

        let mut min = Coord::max();
        let mut max = Coord::min();

        for room in self {
            let (room_min, room_max) = room.get_min_max_coords();

            min.x = min.x.min(room_min.x);
            min.y = min.y.min(room_min.y);

            max.x = max.x.max(room_max.x);
            max.y = max.y.max(room_max.y);
        }

        println!("Min: {} {} Max: {} {}", min.x, min.y, max.x, max.y);

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let c = Coord { x, y };
                if self.iter().any(|r| r.tiles.contains(&c)) {
                    print!("X ");
                } else {
                    print!("  ");
                }
            }
            println!("");
        }

        // for x in min.y..=max.y {
        //     for y in min.x..=max.x {
        //         let c = Coord { x, y };
        //         if self.iter().any(|r| r.tiles.contains(&c)) {
        //             print!("X ");
        //         } else {
        //             print!("  ");
        //         }
        //     }
        //     println!("");
        // }
    }
}

