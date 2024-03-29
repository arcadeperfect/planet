use anyhow::Result;
use image::{Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;


use crate::{
    noise_circle::generate_fbm_circle,
    types::{Coord, FMap, FractalNoiseOptions, UMap8},
    PlanetOptions,
};

pub fn get_initial_planet_map(
    options: &PlanetOptions,
    fractal_options: Vec<&FractalNoiseOptions>,
) -> Result<(UMap8, FMap, FMap)> {
    let (map, field, depth) = generate_fbm_circle(
        options.radius,
        options.resolution,
        fractal_options,
        options.mask_frequency,
        options.mask_z,
        options.global_amplitude,
        options.displacement_scale,
        options.displacement_frequency,
    )?;

    Ok((map, field, depth))
}
pub fn umap_to_image_buffer(input: &UMap8) -> RgbaImage {


    // input.debug_print();

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

            // Create an Rgba pixel with the same value for R, G, and B, and full opacity for alpha
            let pixel = Rgba([color_value, color_value, color_value, 255]);

            // Place the pixel in the corresponding position in the image
            // Note: the y coordinate is calculated as (height - 1 - y) to flip the image vertically
            // image.put_pixel((width -1 - x) as u32, (height - 1 - y) as u32, pixel);
            image.put_pixel((y) as u32, (x) as u32, pixel);
        }
    }

    image
}

pub fn apply_blur(image: &RgbaImage, sigma: f32) -> RgbaImage {
    if sigma < 0.01 {
        return image.clone();
    }

    let sigma = sigma.max(0.01);
    let blurred = gaussian_blur_f32(image, sigma);
    let brightest = find_brightest_pixel(&blurred);

    multiply_image_by(&blurred, 255. / brightest.0[0] as f32)
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


pub fn simple_line(start: Coord, end: Coord) -> Vec<Coord> {
    let mut points = Vec::new();

    let mut x0 = start.x as isize;
    let mut y0 = start.y as isize;
    let x1 = end.x as isize;
    let y1 = end.y as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push(Coord {
            x: x0 as usize,
            y: y0 as usize,
        });

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    points
}

pub fn thick_line(start: &Coord, end: &Coord, thickness: usize) -> Vec<Coord> {
    let mut points = Vec::new();
    let mut x0 = start.x as isize;
    let mut y0 = start.y as isize;
    let x1 = end.x as isize;
    let y1 = end.y as isize;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let half_thickness = thickness as isize / 2;

    loop {
        for i in -half_thickness..=half_thickness {
            for j in -half_thickness..=half_thickness {
                let px = x0 + i;
                let py = y0 + j;
                if px >= 0 && py >= 0 {
                    points.push(Coord {
                        x: px as usize,
                        y: py as usize,
                    });
                }
            }
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    points
}


pub fn nearest_neighbor(a: Vec<Coord>, b: Vec<Coord>) -> (Coord, Coord){
    let mut dist = f32::MAX;
    let mut s_a = a[0];
    let mut s_b = b[0];
    for c_a in &a{
        for c_b in &b{
            let d = dist_squared(&c_a, &c_b);
            if d < dist {
                dist = d;
                s_a = *c_a;
                s_b = *c_b;
            }
        }
    }
    (s_a, s_b)
}




fn dist_squared(a: &Coord, b: &Coord) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy) as f32
}