use image::{Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;
use noise::{core::open_simplex::open_simplex_3d, permutationtable::PermutationTable};
use anyhow::Result;

use crate::{
    types::{Blank, FMap, PlanetMap, UMap16, UMap8},
    utils::{ang, circular_coords, dist, lerp},
    PlanetOptions,
};

pub fn get_initial_planet_map(options: &PlanetOptions, hasher: &PermutationTable) -> Result<(UMap8, FMap)> {
        
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
) -> Result<(UMap8, FMap)> {
    let radius = resolution as f32 * 0.4 * radius as f32;
    let center = (resolution / 2, resolution / 2);
    let f1: f32 = 0.;
    let f2: f32 = 15.;
    let f = lerp(f1, f2, freq);

    let mut map = UMap8::blank(resolution as usize);
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

            map[x as usize][y as usize] = (dist < (radius + noise_offset)) as u8;
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

pub fn umap_to_rgba(input: &UMap8) -> RgbaImage {
    // Determine the dimensions of the input matrix
    let height = input.len();
    let width = if height > 0 { input[0].len() } else { 0 };

    // Create a new RgbaImage with the same dimensions
    let mut image = RgbaImage::new(width as u32, height as u32);

    for (y, row) in input.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Ensure the value is either 0 or 1
            assert!(value == 0 || value == 1, "Input values must be 0 or 1");

            // Multiply by 255 to get the actual color value
            let color_value = value * 255;
            // let color_value = value;

            // Create an Rgba pixel with the same value for R, G, and B, and full opacity for alpha
            let pixel = Rgba([color_value, color_value, color_value, 255]);

            // Place the pixel in the corresponding position in the image
            image.put_pixel(x as u32, y as u32, pixel);
        }
    }

    image
}

pub fn apply_blur(image: &RgbaImage, sigma: f32) -> RgbaImage {

    if sigma < 0.01{
        return image.clone();
    }

    let sigma = sigma.max(0.01);
    gaussian_blur_f32(image, sigma)
}