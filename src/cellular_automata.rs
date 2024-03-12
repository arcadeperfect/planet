use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::{types::{Blank, FMap, UMap8}, PlanetOptions};


use rayon::prelude::*;



pub fn simulate(options: &PlanetOptions, _map: &UMap8, altitude: &FMap) -> UMap8 {
    let thresh: u32 = options.thresh;
    let mut map1: UMap8 = random_distribution(options.resolution(), options.weight);
    let mut map2 = UMap8::blank(options.resolution() as usize);

    for _i in 1..options.iterations {
        map2.par_iter_mut().enumerate().for_each(|(y, row)| {
            for xx in 0..options.resolution() {
                let x = xx as usize;
                let n = get_neighboring_wall_tile_count(&x, &(y as usize), &map1);

                let distance = altitude[y][x as usize];
                let distance = distance / options.resolution as f32;

                if n > thresh {
                    row[x as usize] = 1;
                } else {
                    if distance < 0.9 {
                        row[x as usize] = 0;
                    } else {
                        row[x as usize] = 1;
                    }
                }
            }
        });

        std::mem::swap(&mut map1, &mut map2);
    }

    map1
}


// pub fn simulate(options: &PlanetOptions, _map: &UMap8, altitude: &FMap) -> UMap8 {
  
//     let thresh: u32 = options.thresh;
//     let mut map1: UMap8 = random_distribution(options.resolution(), options.weight);
//     let mut map2 = UMap8::blank(options.resolution() as usize);

//     for _i in 1..options.iterations {
//         for yy in 0..options.resolution() {
//             for xx in 0..options.resolution() {
                
//                 let x = xx as usize;
//                 let y = yy as usize;

//                 let n = get_neighboring_wall_tile_count(&x, &y, &map1);

//                 let distance = altitude[x][y];
//                 let distance = distance / options.resolution as f32;
                
//                 if n > thresh {
//                     map2[y as usize][x as usize] = 1;
//                 } else {
//                     if distance < 0.9 {
//                         map2[y as usize][x as usize] = 0;
//                     } else {
//                         map2[y as usize][x as usize] = 1;
//                     }
//                 }
//             }
//         }

//         std::mem::swap(&mut map1, &mut map2);
//     }

//     map1
// }


fn get_neighboring_wall_tile_count(x: &usize, y: &usize, img: &Vec<Vec<u8>>) -> u32 {
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