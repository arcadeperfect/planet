use std::collections::HashSet;
use anyhow::Result;

use image::{Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;


use crate::{
    noise_circle::generate_fbm_circle, types::{Blank, Coord, FMap, FractalNoiseOptions, UMap8}, PlanetOptions
};

pub fn get_initial_planet_map(
    options: &PlanetOptions,
    fractal_options: Vec<&FractalNoiseOptions>,
) -> Result<(UMap8, FMap, FMap, FMap)> {
    let (map, field, depth) = generate_fbm_circle(
        options.radius,
        options.resolution,
        fractal_options,
        options.noise_mask_options.mask_frequency,
        options.noise_mask_options.mask_z,
        options.global_noise_options.amplitude,
        options.displacement_scale,
        options.displacement_frequency,
        options.global_noise_options.frequency,
    )?;

    let surface_distance_field = get_surface_distance_field(&map, &get_surface(&map));



    Ok((map, field, depth, surface_distance_field))
}
pub fn umap_to_image_buffer(input: &UMap8) -> RgbaImage {

    let height = input.len();
    let width = if height > 0 { input[0].len() } else { 0 };
    let mut image = RgbaImage::new(width as u32, height as u32);

    for (y, row) in input.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            assert!(value == 0 || value == 1, "Input values must be 0 or 1");
            let color_value = value * 255;
            let pixel = Rgba([color_value, color_value, color_value, 255]);
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

pub fn max_inscribed_circle(tiles: &[Coord], edges: &[usize]) -> Coord {

    let edges_hash: HashSet<Coord> = edges.iter().map(|&i| tiles[i]).collect();
    let mut center = Coord::default();
    let mut max_min_d = f32::MIN;

    if edges_hash.len() == tiles.len() {
        let sum = tiles
            .iter()
            .fold((0, 0), |acc, coord| (acc.0 + coord.x, acc.1 + coord.y));
        let count = tiles.len();
        return Coord {
            x: sum.0 / count,
            y: sum.1 / count,
        };
    }

    for &tile in tiles.iter().filter(|&&t| !edges_hash.contains(&t)) {
        let min_d = edges
            .iter()
            .filter_map(|&i| {
                let distance = dist_squared(&tile, &tiles[i]);
                if distance.is_nan() {
                    None
                } else {
                    Some(distance)
                }
            })
            .fold(f32::INFINITY, f32::min); // We want the minimum distance to the edges

        if min_d > max_min_d {
            max_min_d = min_d;
            center = tile;
        }
    }
    center
}

pub fn average_center(tiles: &[Coord]) -> Coord {

    let coord_average = tiles.iter().fold((0, 0), |acc, coord| (acc.0 + coord.x, acc.1 + coord.y));
    let count = tiles.len();
    Coord {
        x: coord_average.0 / count,
        y: coord_average.1 / count,
    }
}

pub fn edge_average_center(tiles: &[Coord], edges: &[usize]) -> Coord {

    let z: Vec<Coord> = edges.iter().map(|&i| tiles[i]).collect();

    let coord_average = z.iter().fold((0, 0), |acc, coord| (acc.0 + coord.x, acc.1 + coord.y));
    let count = tiles.len();
    Coord {
        x: coord_average.0 / count,
        y: coord_average.1 / count,
    }
}

pub fn dist_squared(a: &Coord, b: &Coord) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy) as f32
}

pub fn dist(a: &Coord, b: &Coord) -> f32 {

    let _dx = b.x as i32 - a.x as i32;
    let _dy = b.y as i32 - a.y as i32;

    let dx = _dx.abs();
    let dy = _dy.abs();

    // let dx = b.x - a.x;
    // let dy = b.y - a.y;
    ((dx * dx + dy * dy) as f32).sqrt()
}

fn get_surface_distance_field(map: &UMap8, surface: &Vec<Coord>) -> FMap {

    let mut out = FMap::blank(map.len());

    for x in 0..map.len() {
        for y in 0..map.len() {
            let tile = map[x][y];
            if tile == 0 {
                continue;
            }

            let mut min_dist = f32::MAX;
            // let mut closest: Coord;

            for coord in surface{
                
                let dist = dist(&Coord{x: x, y: y}, &coord);

                if dist < min_dist {
                    min_dist = dist;
                    // closest = coord.clone();
                }
            }

            // println!("{}", min_dist);
            out[x][y] = min_dist;
            
        }
    }
    out
}


fn get_surface(map: &UMap8) -> Vec<Coord> {
    let mut out = Vec::new();

    for x in 0..map.len() {
        for y in 0..map.len() {
            let _v = map[x][y];
            if check_neighbors_horizonatl_or_vertical(x, y, &map) {
                out.push(Coord { x, y });
            }
        }
    }
    out
}

fn check_neighbors_horizonatl_or_vertical(x: usize, y: usize, map: &UMap8) -> bool {
    if map[x][y] == 0 {
        return false;
    }

    // left
    if x > 0 {
        if map[x - 1][y] == 0 {
            return true;
        }
    }

    // right
    if x < map.len() - 1 {
        if map[x + 1][y] == 0 {
            return true;
        }
    }

    // above
    if y > 0 {
        if map[x][y - 1] == 0 {
            return true;
        }
    }

    // below

    if y < map.len() - 1 {
        if map[x][y + 1] == 0 {
            return true;
        }
    }

    false
}
