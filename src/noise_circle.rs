use anyhow::Result;
use noise::{permutationtable::PermutationTable, Fbm, MultiFractal, NoiseFn, Perlin};
use std::f32::consts::PI;

use crate::{
    types::{Blank, FMap, FractalNoiseOptions, UMap8},
    utils::{ang, circular_coord, dist, mapf64, CircleIterator},
};

pub fn generate_fbm_circle(
    radius: f32,
    resolution: u32,
    noise_options: Vec<&FractalNoiseOptions>,
    mask_frequency: f64,
    mask_z: f64,
    global_amplitude: f32,
    displacement_scale: f64,
    displacement_frequency: f64,
) -> Result<(UMap8, FMap, FMap)> {
    let radius = resolution as f32 * 0.4 * radius as f32;
    let center = (resolution / 2, resolution / 2);
    let f1: f32 = 0.;
    let f2: f32 = 15.;
    // let f = lerp(f1, f2, freq);

    let mut map = UMap8::blank(resolution as usize);
    let mut altitude_field: FMap = FMap::blank(resolution as usize);
    let mut depth_field: FMap = FMap::blank(resolution as usize);

    // for (i, option) in noise_options.iter().enumerate() {
    //     println!("----");
    //     println!("{:?}", option);
    // }

    let combiner = FbmCombiner::new(
        noise_options, 
        0,
        displacement_scale,
        displacement_frequency
    );

    // let fbm = Fbm::<Perlin>::new(0)
    //     .set_frequency(noise_options[0].frequency)
    //     .set_persistence(noise_options[0].persistence)
    //     .set_lacunarity(noise_options[0].lacunarity)
    //     .set_octaves(noise_options[0].octaves);

    for x in 0..resolution {
        for y in 0..resolution {
            let s = ang((x, y), center);
            let (a, b) = circular_coord(s, 1.);

            // let amplitude = noise_options[0].amplitude * 0.3 * resolution as f32;

            let noise_offset = combiner.get([a as f64, b as f64], mask_frequency, mask_z) as f32
                * resolution as f32
                * 0.5
                * global_amplitude;

            let dist = dist(center, (x, y));
            let altitude = dist / radius;
            let depth = dist / (radius + noise_offset);

            // println!("{} {}", dist, radius);

            map[x as usize][y as usize] = (dist < (radius + noise_offset)) as u8;
            altitude_field[x as usize][y as usize] = altitude;
            depth_field[x as usize][y as usize] = depth;
        }
    }

    Ok((map, altitude_field, depth_field))
}

struct FbmCombiner {
    fbm_vec: Vec<Fbm<Perlin>>,
    mask_noise: Perlin,
    displacement_noise_x: Perlin,
    displacement_noise_y: Perlin,
    displacement_scale: f64,
    displacement_frequency: f64,
    amplitudes: Vec<f32>,
}

impl FbmCombiner {
    fn new(
        options_vec: Vec<&FractalNoiseOptions>,
        seed: u32,
        displacement_scale: f64,
        displacement_frequency: f64,
    ) -> Self {
        let displacement_noise_x = Perlin::new(seed + 1);
        let displacement_noise_y = Perlin::new(seed + 2);

        FbmCombiner {
            fbm_vec: options_vec
                .iter()
                .map(|x| {
                    Fbm::<Perlin>::new(seed)
                        .set_frequency(x.frequency)
                        .set_persistence(x.persistence)
                        .set_lacunarity(x.lacunarity)
                        .set_octaves(x.octaves)
                })
                .collect(),
            mask_noise: Perlin::new(seed),
            displacement_noise_x,
            displacement_noise_y,
            displacement_scale,
            displacement_frequency,
            amplitudes: options_vec.iter().map(|x| x.amplitude).collect(),
        }
    }

    fn get(&self, point: [f64; 2], mask_freq: f64, mask_z: f64) -> f64 {
        let displaced_point = [
            point[0]
                + self.displacement_noise_x.get([
                    point[0] * self.displacement_frequency,
                    point[1] * self.displacement_frequency,
                ]) * self.displacement_scale,
            point[1]
                + self.displacement_noise_y.get([
                    point[0] * self.displacement_frequency,
                    point[1] * self.displacement_frequency,
                ]) * self.displacement_scale,
        ];

        match self.fbm_vec.len() {
            0 => 0.0,
            1 => self.fbm_vec[0].get(point) * self.amplitudes[0] as f64,
            _ => {
                let interval = 1.0 / (self.fbm_vec.len() - 1) as f64;
                let mask_point = [mask_freq * point[0], mask_freq * point[1] + mask_z];
                let mask_value = self.mask_noise.get(mask_point);
                let mask = mapf64(mask_value, -1.0, 1.0, 0.0, 1.0);

                let mut total_value = 0.0;

                for (i, fbm) in self.fbm_vec.iter().enumerate() {
                    if i < self.fbm_vec.len() - 1 {
                        // Calculate the blend factor based on the mask's position within the interval
                        let lower_bound = i as f64 * interval;
                        let upper_bound = (i + 1) as f64 * interval;
                        if mask >= lower_bound && mask < upper_bound {
                            let blend_factor = (mask - lower_bound) / interval;
                            let noise1 = fbm.get(displaced_point) * self.amplitudes[i] as f64;
                            let noise2 = self.fbm_vec[i + 1].get(displaced_point)
                                * self.amplitudes[i + 1] as f64;
                            // Linear interpolation between the two noise values
                            total_value += noise1 * (1.0 - blend_factor) + noise2 * blend_factor;
                            break;
                        }
                    } else if mask >= i as f64 * interval {
                        // Handle the last interval
                        total_value += fbm.get(point) * self.amplitudes[i] as f64;
                    }
                }

                total_value
            }
        }
    }

    // fn get(&self, point: [f64; 2], mask_freq: f64, mask_z: f64) -> f64 {
    //     match self.fbm_vec.len() {
    //         0 => 0.0,
    //         1 => self.fbm_vec[0].get(point) * self.amplitudes[0] as f64,
    //         _ => {

    //             let interval = 1.0 / self.fbm_vec.len() as f64;
    //             let mut total_value = 0.0;
    //             let mask_point = [mask_freq * point[0], mask_freq * point[1] + mask_z];
    //             let mask_value = self.mask_noise.get(mask_point);
    //             let mask = mapf64(mask_value, -1.0, 1.0, 0.0, 1.0);

    //             let mut n1 = 0.;
    //             let mut n2 = 0.;
    //             let mut n3 = 0.;

    //             for (i, fbm) in self.fbm_vec.iter().enumerate() {

    //                 let mut v: f64 = 0.;

    //                 if(mask > i as f64 * interval && mask <= (i+1) as f64 * interval){
    //                     let noise = fbm.get(point) * self.amplitudes[i] as f64;
    //                     v += noise;
    //                 }

    //                 let noise = fbm.get(point) * self.amplitudes[i] as f64;

    //                 total_value += v;

    //             }

    //             total_value

    //         }
    //     }

    //     // let mask_value = self.mask_noise.get(point).abs(); // Ensure mask_value is between 0 and 1
    //     // let intervals = 1.0 / self.options_vec.len() as f64;

    //     // let mut total_value = 0.0;
    //     // let mut accumulated_intervals = 0.0;

    //     // for (i, fbm) in self.options_vec.iter().enumerate() {
    //     //     if mask_value <= (i as f64 + 1.0) * intervals {
    //     //         let local_interval = mask_value - accumulated_intervals;
    //     //         let weight = local_interval / intervals;
    //     //         total_value += fbm.get(point) * weight * self.amplitudes[i] as f64;
    //     //         break;
    //     //     } else {
    //     //         total_value += fbm.get(point) * intervals;
    //     //     }
    //     //     accumulated_intervals += intervals;
    //     // }

    //     // total_value
    // }
}
