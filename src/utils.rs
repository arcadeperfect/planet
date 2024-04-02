use glam::Vec2;
use num_traits::{AsPrimitive, Float};
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use std::f32::consts::PI;

use crate::types::FMap;

pub fn circle(center: Vec2, radius: f32, num_segments: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(num_segments);

    for i in 0..=num_segments {
        let angle = 2.0 * PI * (i as f32) / (num_segments as f32);
        let x = radius * angle.cos() + center.x;
        let y = radius * angle.sin() + center.y;
        points.push(Vec2::new(x, y));
    }

    points
}

pub fn mapf64(value: f64, in_low: f64, in_high: f64, to_low: f64, to_high: f64) -> f64 {
    (value - in_low) / (in_high - in_low) * (to_high - to_low) + to_low
}

pub fn mapf32(value: f32, in_low: f32, in_high: f32, to_low: f32, to_high: f32) -> f32 {
    (value - in_low) / (in_high - in_low) * (to_high - to_low) + to_low
}

pub fn lerp<T: Float>(a: T, b: T, t: T) -> T {
    a + t * (b - a)
}

// return angle between two points in radians
pub fn ang(a: (u32, u32), b: (u32, u32)) -> f32 {
    let dx = b.0 as f32 - a.0 as f32;
    let dy = b.1 as f32 - a.1 as f32;
    dy.atan2(dx)
}

// return a circle of coords for sampling 2d noise in a repeatable fashion
pub fn circular_coord(angle_in_radians: f32, scale: f32) -> (f32, f32) {
    let x = angle_in_radians.cos() * scale;
    let y = angle_in_radians.sin() * scale;
    (x, y)
}

pub struct CircleIterator {
    angle: f32,
    step: usize,
    total_steps: usize,
    scale: f32,
    angle_increment: f32,
}

impl CircleIterator {
    pub fn new(total_steps: usize, scale: f32) -> Self {
        CircleIterator {
            angle: 0.0,
            step: 0,
            total_steps,
            scale,
            angle_increment: 2.0 * std::f32::consts::PI / total_steps as f32,
        }
    }
}

impl Iterator for CircleIterator {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.step >= self.total_steps {
            None
        } else {
            let point = circular_coord(self.angle, self.scale);
            self.angle += self.angle_increment;
            self.step += 1;
            Some(point)
        }
    }
}

// return a random distribution of 0 and 1
pub fn random_distribution(resolution: u32, weight: f32) -> Vec<Vec<u8>> {
    let mut img: Vec<Vec<u8>> = vec![vec![0; resolution as usize]; resolution as usize];
    let mut rng = StdRng::seed_from_u64(1);

    for y in 0..resolution {
        for x in 0..resolution {
            let random_value: f32 = rng.gen(); // Generates a float between 0 and 1.
            img[y as usize][x as usize] = if random_value < weight { 1 } else { 0 };
        }
    }

    img
}


pub fn random_distribution_mask_weighted(resolution: u32, weight: f32, mask: &Vec<Vec<f32>>, invert_mask: bool, seed: u64) -> Vec<Vec<u8>> {
    let mut img: Vec<Vec<u8>> = vec![vec![0; resolution as usize]; resolution as usize];
    let mut rng = StdRng::seed_from_u64(seed);
    let range = Uniform::new(0.0, 1.0);

    for y in 0..resolution {
        for x in 0..resolution {
            let random_value: f32 = rng.sample(&range);
            let mask_value = mask[x as usize][y as usize];
            let adjusted_weight = if invert_mask {
                if mask_value > 0.0 {
                    weight * (1.0 + mask_value)
                } else {
                    weight * (1.0 - mask_value)
                }
            } else {
                if mask_value > 0.0 {
                    weight * (1.0 - mask_value)
                } else {
                    weight * (1.0 + mask_value)
                }
            };
            img[y as usize][x as usize] = if random_value < adjusted_weight { 1 } else { 0 };
        }
    }

    img
}

pub fn dist_squared<T: AsPrimitive<f32>>(a: (T, T), b: (T, T)) -> u32 {
    let dx = b.0.as_() - a.0.as_();
    let dy = b.1.as_() - a.1.as_();

    (dx * dx + dy * dy) as u32
}

pub fn dist<T: AsPrimitive<f32>>(a: (T, T), b: (T, T)) -> f32 {
    let dx = b.0.as_() - a.0.as_();
    let dy = b.1.as_() - a.1.as_();

    (dx * dx + dy * dy).sqrt()
}

pub fn doubler<T: Clone>(input: Vec<T>) -> Option<Vec<(T, T)>> {
    if input.len() < 2 {
        return None;
    }

    let tuples: Vec<(T, T)> = input
        .windows(2)
        .map(|window| (window[0].clone(), window[1].clone()))
        .collect();

    Some(tuples)
}