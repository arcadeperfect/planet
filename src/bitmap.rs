use image::{Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;
use noise::{core::open_simplex::open_simplex_3d, permutationtable::PermutationTable};
use anyhow::Result;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    noise_circle::generate_fbm_circle, types::{Blank, FMap, FractalNoiseOptions, UMap8}, utils::{ang, circular_coord, dist, lerp}, PlanetOptions
};

pub fn get_initial_planet_map(options: &PlanetOptions, fractal_options: Vec<&FractalNoiseOptions>,hasher: &PermutationTable) -> Result<(UMap8, FMap, FMap)> {
        
    // let (map, field, depth) = generate_noise_circle(
    //     options.radius,
    //     options.resolution,
    //     options.frequency,
    //     options.amplitude,
    //     hasher,
    // )?;
    
    // let frequency = (options.frequency * 100.) as f64;
    // let lacunarity = 1.;
    // let octaves = 8;
    // let persistence = 0.5;

    // let foptions = FractalNoiseOptions{
    //     frequency,
    //     lacunarity,
    //     octaves,
    //     persistence
    // };

    // let mut optionsVec: Vec<&FractalNoiseOptions> = Vec::new();
    // optionsVec.push(fractal_options);

;

    let (map, field, depth) = generate_fbm_circle(
        options.radius,
        options.resolution,
        // fractal_options.amplitude * 30.,
        fractal_options,
        options.mask_frequency,
        options.mask_z,
        options.global_amplitude,
        options.displacement_scale,
        options.displacement_frequency,
    )?;


    Ok((map, field, depth))
}
// pub fn generate_noise_circle(
//     radius: f32,
//     resolution: u32,
//     freq: f32,
//     amplitude: f32,
//     hasher: &PermutationTable,
// ) -> Result<(UMap8, FMap, FMap)> {

//     let radius = resolution as f32 * 0.4 * radius as f32;
//     let center = (resolution / 2, resolution / 2);
//     let f1: f32 = 0.;
//     let f2: f32 = 15.;
//     let f = lerp(f1, f2, freq);

//     let mut map = UMap8::blank(resolution as usize);
//     let mut altitude_field: FMap = FMap::blank(resolution as usize);
//     let mut depth_field: FMap = FMap::blank(resolution as usize);

//     for x in 0..resolution {
//         for y in 0..resolution {
//             let s = ang((x, y), center);
//             let (a, b) = circular_coord(s, 1.);
//             let noise_offset = open_simplex_3d([(a * f) as f64, (b * f) as f64, 0.], hasher)
//                 as f32
//                 * 30.0
//                 * amplitude;

//             let dist = dist(center, (x, y));
//             let altitude = dist / radius;
//             let depth = dist / (radius + noise_offset);

            

//             let _b = (dist < (radius + noise_offset)) as u16;

//             println!("{} {}", dist, radius);

//             map[x as usize][y as usize] = (dist < (radius + noise_offset)) as u8;
//             altitude_field[x as usize][y as usize] = altitude;
//             depth_field[x as usize][y as usize] = depth;
    
//         }
//     }

//     Ok((map, altitude_field, depth_field))

// }

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
