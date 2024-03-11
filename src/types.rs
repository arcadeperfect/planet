use std::sync::Arc;

use glam::Vec2;
use image::{ImageBuffer, Rgba};

pub type FloatImage = ImageBuffer<Rgba<f32>, Vec<f32>>;
// pub type Map16 = Vec<Vec<u16>>;
// pub type Map8 = Vec<Vec<u8>>;

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
    resolution: usize,
    pub main: Option<UMap8>,
    pub rooms: Option<UMap8>,
    pub edges: Option<UMap8>,
    pub altitude: Option<FMap>,
    pub depth: Option<FMap>,
    pub edge_distance_field: Option<FMap>,
}

impl PlanetMap {
    pub fn blank(resolution: usize) -> Self {
        PlanetMap {
            resolution,
            main: Some(vec![vec![0; resolution]; resolution]),
            rooms: Some(vec![vec![0; resolution]; resolution]),
            edges: Some(vec![vec![0; resolution]; resolution]),
            altitude: Some(vec![vec![0.; resolution]; resolution]),
            depth: Some(vec![vec![0.; resolution]; resolution]),
            edge_distance_field: Some(vec![vec![0.; resolution]; resolution]),
        }
    }

    pub fn empty(resolution: usize) -> Self {
        PlanetMap {
            resolution,
            main: None,
            rooms: None,
            edges: None,
            altitude: None,
            depth: None,
            edge_distance_field: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlanetOptions {
    pub seed: u32,
    pub frequency: f32,
    pub amplitude: f32,
    pub radius: f32,
    pub resolution: u32,
    pub weight: f32,
    pub thresh: u32,
    pub iterations: u32,
    pub blur: f32,
    // pub distance_pow: f32,
    pub min_room_size: usize,
}

impl PlanetOptions {
    pub fn new(
        seed: u32,
        frequency: f32,
        amplitude: f32,
        radius: f32,
        resolution: u32,
        thresh: u32,
        iterations: u32,
        weight: f32,
        blur: f32,
        distance_pow: f32,
        min_size: usize,
    ) -> Self {
        Self {
            seed,
            frequency,
            amplitude,
            radius,
            resolution: resolution.max(8),
            thresh,
            iterations,
            weight,
            blur,
            // distance_pow,
            min_room_size: min_size,
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
            frequency: 1.,
            amplitude: 1.,
            radius: 1.,
            resolution: 100,
            weight: 50.,
            thresh: 4,
            iterations: 10,
            blur: 1.,
            // distance_pow: 2.,
            min_room_size: 20,
        }
    }
}

