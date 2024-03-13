use anyhow::Result;
use noise::{permutationtable::PermutationTable, Fbm, MultiFractal, NoiseFn, Perlin};
use std::f32::consts::PI;

use crate::{
    types::{Blank, FMap, FractalNoiseOptions, UMap8},
    utils::{ang, circular_coord, dist, CircleIterator},
};

pub fn generate_fbm_circle(
    radius: f32,
    resolution: u32,
    noise_options: Vec<&FractalNoiseOptions>,
) -> Result<(UMap8, FMap, FMap)> {
    let radius = resolution as f32 * 0.4 * radius as f32;
    let center = (resolution / 2, resolution / 2);
    let f1: f32 = 0.;
    let f2: f32 = 15.;
    // let f = lerp(f1, f2, freq);

    let mut map = UMap8::blank(resolution as usize);
    let mut altitude_field: FMap = FMap::blank(resolution as usize);
    let mut depth_field: FMap = FMap::blank(resolution as usize);

    if noise_options.len() == 0 {
        for x in 0..resolution {
            for y in 0..resolution {
                let s = ang((x, y), center);
                let (a, b) = circular_coord(s, 1.);

                // let noise_offset =
                //     fbm.get([a as f64, b as f64]) as f32 * noise_options[0].amplitude;

                let dist = dist(center, (x, y));
                let altitude = dist / radius;
                let depth = dist / (radius);

                // println!("{} {}", dist, radius);

                map[x as usize][y as usize] = (dist < (radius)) as u8;
                altitude_field[x as usize][y as usize] = altitude;
                depth_field[x as usize][y as usize] = depth;
            }
        }

        Ok((map, altitude_field, depth_field))
    } else {
        let fbm = Fbm::<Perlin>::new(0)
            .set_frequency(noise_options[0].frequency)
            .set_persistence(noise_options[0].persistence)
            .set_lacunarity(noise_options[0].lacunarity)
            .set_octaves(noise_options[0].octaves);

        for x in 0..resolution {
            for y in 0..resolution {
                let s = ang((x, y), center);
                let (a, b) = circular_coord(s, 1.);

                let amplitude = noise_options[0].amplitude * 0.3 * resolution as f32;

                let noise_offset =
                    fbm.get([a as f64, b as f64]) as f32 * amplitude;

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
}

// pub struct FractalNoiseOptions {
//     pub frequency: f64,
//     pub lacunarity: f64,
//     pub octaves: usize,
//     pub persistence: f64,
// }
