use noise::{core::open_simplex::open_simplex_3d, permutationtable::PermutationTable};
use anyhow::Result;

use crate::{
    types::{Blank, FMap, PlanetMap, UMap},
    utils::{ang, circular_coords, dist, lerp},
    PlanetOptions,
};

pub fn get_initial_planet_map(options: &PlanetOptions, hasher: &PermutationTable) -> Result<(UMap, FMap)> {
    
    println!("gettin");
    
    let (map, field) = generate_noise_circle(
        options.radius,
        options.resolution,
        options.frequency,
        options.amplitude,
        hasher,
    )?;
    Ok((map, field))
}
pub fn generate_noise_circle(
    radius: f32,
    resolution: u32,
    freq: f32,
    amplitude: f32,
    hasher: &PermutationTable,
) -> Result<(UMap, FMap)> {
    let radius = resolution as f32 * 0.4 * radius as f32;
    let center = (resolution / 2, resolution / 2);
    let f1: f32 = 0.;
    let f2: f32 = 15.;
    let f = lerp(f1, f2, freq);

    let mut map: UMap = UMap::blank(resolution as usize);
    let mut field: FMap = FMap::blank(resolution as usize);

    for x in 0..resolution {
        for y in 0..resolution {
            let s = ang((x, y), center);
            let (a, b) = circular_coords(s, 1.);
            let noise_offset = open_simplex_3d([(a * f) as f64, (b * f) as f64, 0.], hasher)
                as f32
                * 30.0
                * amplitude;

            let dist = dist(center, (x, y));



            let _altitude = dist / radius;
            let _depth = dist / (radius + noise_offset);

            // set planet tile to 1 if within the radius plus the noise offset

            let _b = (dist < (radius + noise_offset)) as u16;

            // println!("{:?}", b);

            map[x as usize][y as usize] = (dist < (radius + noise_offset)) as u16;
            field[x as usize][y as usize] = dist;
            // // set the altitude as the distance from the center normailzed to 1 at the nominal radius
            // map.altitude[x as usize][y as usize] = altitude;

            // // set the depth as the distance from the tile to the actual surface after the noise offset
            // map.depth[x as usize][y as usize] = depth;

            // /* note we don't bother to set the distance field in the map as that's expensive and it should be done after rooms are generated */
        }
    }

    Ok((map, field))

    // for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    //     let s = ang((x, y), center);
    //     let (a, b) = circular_coords(s, 1.);
    //     let n = open_simplex_3d([(a * f) as f64, (b * f) as f64, 0.], &hasher) as f32
    //         * 30.0
    //         * amplitude;

    //     let dist = dist(center, (x, y));

    //     let d1 = dist / radius;
    //     let d2 = dist / (radius + n);

    //     if dist < (radius + n) {
    //         *pixel = Rgba([1.0, d1, d2, 1.0]);
    //     } else {
    //         *pixel = Rgba([0.0, d1, d2, 1.0]);
    //     }
    // }

    // ()
}
