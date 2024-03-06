use glam::Vec2;

pub struct PlumbetPlugin;

mod utils;

pub struct Planet {
    pub contours: Vec<Vec<Vec2>>,
}

pub struct PlanetBuilder {}

impl PlanetBuilder {
    pub fn new(options: PlanetOptions) -> Planet {
        Planet {
            contours: PlanetBuilder::temp_contours(&options),
        }
    }

    fn temp_contours(options: &PlanetOptions) -> Vec<Vec<Vec2>> {
        let radius = options.radius;

        vec![
            utils::circle(Vec2::new(0., 0.), radius, options.resolution as usize),
            utils::circle(Vec2::new(0., 0.), radius / 2., options.resolution as usize),
        ]
    }
}

pub struct PlanetOptions {
    frequency: f32,
    amplitude: f32,
    radius: f32,
    resolution: u32,
    weight: f32,
    thresh: u32,
    iterations: u32,
    blur: f32,
    distance_pow: f32,
    min_room_size: usize,
}

impl PlanetOptions {
    pub fn new(
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
            frequency,
            amplitude,
            radius,
            resolution: resolution.max(8),
            thresh,
            iterations,
            weight,
            blur,
            distance_pow,
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
            frequency: 1.,
            amplitude: 1.,
            radius: 1.,
            resolution: 100,
            weight: 50.,
            thresh: 4,
            iterations: 10,
            blur: 1.,
            distance_pow: 2.,
            min_room_size: 20,
        }
    }
}
