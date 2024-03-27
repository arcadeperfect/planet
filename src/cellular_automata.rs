use crate::{
    types::{Blank, FMap, UMap8},
    PlanetOptions,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

use rayon::prelude::*;

fn precompute_circle_offsets(radius: u32) -> Vec<(i32, i32)> {
    let mut offsets = Vec::new();

    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let distance_squared = dx * dx + dy * dy;
            if distance_squared <= (radius * radius) as i32 {
                offsets.push((dx, dy));
            }
        }
    }

    offsets
}

pub fn simulate(options: &PlanetOptions, _map: &UMap8, altitude: &FMap) -> UMap8 {
    let mut map1: UMap8 = random_distribution(options.resolution(), options.weight);
    let mut map2 = UMap8::blank(options.resolution() as usize);

    let iters = if options.ca_iterations % 2 != 0 {
        options.ca_iterations + 1
    } else {
        options.ca_iterations
    };

    let offsets = precompute_circle_offsets(options.ca_search_radius);

    for _i in 1..iters {
        map2.par_iter_mut().enumerate().for_each(|(y, row)| {
            for xx in 0..options.resolution() {
                let x = xx as usize;
                // let n = get_neighboring_wall_tile_count_diagonal(&x, &(y as usize), &map1);

                // let distance = altitude[y][x as usize];

                let d = decision(&x, &(y as usize), &map1, &altitude, &options, &offsets);

                if d {
                    row[x] = 0;
                } else {
                    row[x] = 1;
                }
            }
        });

        std::mem::swap(&mut map1, &mut map2);
    }

    // for _i in 1..iters {
    //     map2.par_iter_mut().enumerate().for_each(|(y, row)| {
    //         row.par_iter_mut().enumerate().for_each(|(x, cell)| {
    //             let d = decision(&x, &(y as usize), &map1, &altitude, &options);
    //             if d {
    //                 *cell = 0;
    //             } else {
    //                 *cell = 1;
    //             }
    //         });
    //     });
    //     std::mem::swap(&mut map1, &mut map2);
    // }

    if options.invert_ca {
        for y in 0..options.resolution() {
            for x in 0..options.resolution() {
                map1[y as usize][x as usize] = if map1[y as usize][x as usize] == 0 {
                    1
                } else {
                    0
                };
            }
        }
    }

    map1
}

fn decision(
    x: &usize,
    y: &usize,
    img: &Vec<Vec<u8>>,
    altitude: &FMap,
    options: &PlanetOptions,
    circle_offsets: &Vec<(i32, i32)>
) -> bool {
    // let result = get_neighboring_wall_tile_count_diagonal(x, y, img);
    // let result = get_neighboring_wall_tile_count_horizontal_and_vertical(x, y, img);

    // println!("{:?}", options.ca_search_radius);
    let result =
        get_neighboring_wall_tile_count_within_radius_circle(x, y, img, options.ca_search_radius, circle_offsets);
    // let result = get_neighboring_wall_tile_count_within_radius_circle(x, y, img, 3);

    let a: i32 = (altitude[*y][*x] as i32) * options.ca_misc;

    // dbg!(a);

    let thresh: i32 = options.thresh as i32 + a;

    result > thresh as u32
}

fn get_neighboring_wall_tile_count_within_radius_circle(
    x: &usize,
    y: &usize,
    img: &Vec<Vec<u8>>,
    radius: u32,
    circle_offsets: &Vec<(i32, i32)>,
) -> u32 {
    let width = img.len() as i32;
    let height = img[0].len() as i32;
    let mut count: u32 = 0;

    let min_x = (*x as i32 - radius as i32).max(0);
    let max_x = (*x as i32 + radius as i32).min(width - 1);
    let min_y = (*y as i32 - radius as i32).max(0);
    let max_y = (*y as i32 + radius as i32).min(height - 1);

    for &(dx, dy) in circle_offsets {
        let nx = *x as i32 + dx;
        let ny = *y as i32 + dy;

        if nx >= min_x && nx <= max_x && ny >= min_y && ny <= max_y {
            let neighbor = img[nx as usize][ny as usize];
            count += neighbor as u32;
        }
    }

    count
}


// fn get_neighboring_wall_tile_count_within_radius_circle(
//     x: &usize,
//     y: &usize,
//     img: &Vec<Vec<u8>>,
//     radius: u32,
// ) -> u32 {
//     let width = img.len() as i32;
//     let height = img[0].len() as i32;
//     let mut count: u32 = 0;

//     for dy in -(radius as i32)..=(radius as i32) {
//         for dx in -(radius as i32)..=(radius as i32) {
//             let distance_squared = dx * dx + dy * dy;
//             if distance_squared > (radius * radius) as i32 {
//                 continue; // Skip pixels outside the circle
//             }

//             let nx = *x as i32 + dx;
//             let ny = *y as i32 + dy;

//             if nx >= 0 && nx < width && ny >= 0 && ny < height {
//                 let neighbor = img[nx as usize][ny as usize];
//                 count += neighbor as u32;
//             }
//         }
//     }

//     count
// }

fn get_neighboring_wall_tile_count_within_radius_square(
    x: &usize,
    y: &usize,
    img: &Vec<Vec<u8>>,
    radius: u32,
) -> u32 {
    let width = img.len() as i32;
    let height = img[0].len() as i32;
    let mut count: u32 = 0;

    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            if dx == 0 && dy == 0 {
                continue; // Skip the center pixel
            }

            let nx = *x as i32 + dx;
            let ny = *y as i32 + dy;

            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                let neighbor = img[nx as usize][ny as usize];
                count += neighbor as u32;
            }
        }
    }

    count
}

fn get_neighboring_wall_tile_count_horizontal_and_vertical(
    x: &usize,
    y: &usize,
    img: &Vec<Vec<u8>>,
) -> u32 {
    let width = img.len() as usize;
    let height = img[0].len() as usize;
    let mut count: u32 = 0;

    // Check left neighbor
    if *x > 0 {
        let neighbor = img[x - 1][*y];
        count += neighbor as u32;
    }

    // Check right neighbor
    if *x < width - 1 {
        let neighbor = img[x + 1][*y];
        count += neighbor as u32;
    }

    // Check top neighbor
    if *y > 0 {
        let neighbor = img[*x][y - 1];
        count += neighbor as u32;
    }

    // Check bottom neighbor
    if *y < height - 1 {
        let neighbor = img[*x][y + 1];
        count += neighbor as u32;
    }

    count
}

fn get_neighboring_wall_tile_count_diagonal(x: &usize, y: &usize, img: &Vec<Vec<u8>>) -> u32 {
    let width = img.len() as u32;
    let height = img[0].len() as u32;

    let mut count: u32 = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip the center pixel
            }

            let nx = *x as i32 + dx;
            let ny = *y as i32 + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let neighbor = img[nx as usize][ny as usize];
                count += neighbor as u32;
            }
        }
    }
    count
}

fn random_distribution(resolution: u32, weight: f32) -> Vec<Vec<u8>> {
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
