use image::{Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;
use noise::{core::open_simplex::open_simplex_3d, permutationtable::PermutationTable};
use anyhow::Result;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    types::{Blank, FMap, UMap8},
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


    let mut map = map.clone();

    // Parallel computation of the map and fields
    let fields: Vec<Vec<f32>> = map
        .par_iter_mut()
        .enumerate()
        .map(|(y, row)| {
            let mut field = vec![0.0; resolution as usize];
            for x in 0..resolution {
                let s = ang((x, y as u32), center);
                let (a, b) = circular_coords(s, 1.);
                let noise_offset = open_simplex_3d([(a * f) as f64, (b * f) as f64, 0.], hasher) as f32 * 30.0 * amplitude;
                let dist = dist(center, (x, y as u32));
                let _altitude = dist / radius;
                let _depth = dist / (radius + noise_offset);
                let _b = (dist < (radius + noise_offset)) as u16;
                row[x as usize] = (dist < (radius + noise_offset)) as u8;
                field[x as usize] = dist;
            }
            field
        })
        .collect();

    // Combine the fields from all threads into a single field vector
    for (y, row) in fields.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            field[x][y] = *value;
        }
    }


    // for x in 0..resolution {
    //     for y in 0..resolution {
    //         let s = ang((x, y), center);
    //         let (a, b) = circular_coords(s, 1.);
    //         let noise_offset = open_simplex_3d([(a * f) as f64, (b * f) as f64, 0.], hasher)
    //             as f32
    //             * 30.0
    //             * amplitude;

    //         let dist = dist(center, (x, y));
    //         let _altitude = dist / radius;
    //         let _depth = dist / (radius + noise_offset);

            

    //         let _b = (dist < (radius + noise_offset)) as u16;

            

    //         map[x as usize][y as usize] = (dist < (radius + noise_offset)) as u8;
    //         field[x as usize][y as usize] = dist;
    //         // // set the altitude as the distance from the center normailzed to 1 at the nominal radius
    //         // map.altitude[x as usize][y as usize] = altitude;

    //         // // set the depth as the distance from the tile to the actual surface after the noise offset
    //         // map.depth[x as usize][y as usize] = depth;

    //         // /* note we don't bother to set the distance field in the map as that's expensive and it should be done after rooms are generated */
    //     }
    // }

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

pub fn umap_to_image_buffer(input: &UMap8) -> RgbaImage {
    // Determine the dimensions of the input matrix
    let height = input.len();
    let width = if height > 0 { input[0].len() } else { 0 };

    // Create a new RgbaImage with the same dimensions
    let mut image = RgbaImage::new(width as u32, height as u32);

    for (y, row) in input.iter().rev().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Ensure the value is either 0 or 1
            assert!(value == 0 || value == 1, "Input values must be 0 or 1");

            // Multiply by 255 to get the actual color value
            let color_value = value * 255;

            // Create an Rgba pixel with the same value for R, G, and B, and full opacity for alpha
            let pixel = Rgba([color_value, color_value, color_value, 255]);

            // Place the pixel in the corresponding position in the image
            // Note: the y coordinate is calculated as (height - 1 - y) to flip the image vertically
            // image.put_pixel((width -1 - x) as u32, (height - 1 - y) as u32, pixel);
            image.put_pixel((x) as u32, (y) as u32, pixel);

        }
    }

    image
}



pub fn apply_blur(image: &RgbaImage, sigma: f32) -> RgbaImage {

    if sigma < 0.01{
        return image.clone();
    }

    let sigma = sigma.max(0.01);
    let blurred = gaussian_blur_f32(image, sigma);

    let brightest = find_brightest_pixel(&blurred);

    multiply_image_by(&blurred, 255./brightest.0[0] as f32)


}

pub fn multiply_image_by(image: &RgbaImage, factor: f32) -> RgbaImage {
    let mut new_image = image.clone();
    for pixel in new_image.pixels_mut() {
        pixel.0[0] = (pixel.0[0] as f32 * factor) as u8;
        pixel.0[1] = (pixel.0[1] as f32 * factor) as u8;
        pixel.0[2] = (pixel.0[2] as f32 * factor) as u8;
    }
    new_image
}


pub fn find_brightest_pixel(image: &RgbaImage) -> Rgba<u8> {
    let mut brightest_pixel = Rgba([0, 0, 0, 0]);
    let mut max_brightness = 0;

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let brightness = pixel.0[0] as u32 + pixel.0[1] as u32 + pixel.0[2] as u32;

            if brightness > max_brightness {
                max_brightness = brightness;
                brightest_pixel = *pixel;
            }
        }
    }

    brightest_pixel
}
