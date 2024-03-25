#![allow(dead_code)]

use anyhow::Result;
use bitmap::{apply_blur, get_initial_planet_map, umap_to_image_buffer};
use cellular_automata::simulate;
use glam::{Vec2, Vec3};
// use tracing::info;
use marching_squares::march_squares_rgba;
use noise::permutationtable::PermutationTable;
use room::Room;
use std::collections::VecDeque;
pub use types::PlanetOptions;
use types::{Coord, FMap, FractalNoiseOptions, PlanetData, PlanetMap, UMap16, UMap8};

use crate::tile_map::{FromUMap, MapDebug, Tile, TileMap, TileMapDebug};

mod bitmap;
mod cellular_automata;
mod marching_squares;
mod noise_circle;
mod noise_example;
pub mod room;
pub mod tile_map;
mod traits;
pub mod types;
mod utils;

pub struct PlanetBuilder {
    hasher: PermutationTable,
}

impl PlanetBuilder {
    pub fn new(seed: u32) -> Self {
        PlanetBuilder {
            hasher: PermutationTable::new(seed),
        }
    }

    pub fn build(
        &self,
        options: PlanetOptions,
        fractal_options: Vec<&FractalNoiseOptions>,
    ) -> Result<PlanetData> {
        tracing::info!("##### new planet #####");

        let (initial_planet_map, altitude_field, depth_field) =
            get_initial_planet_map(&options, fractal_options, &self.hasher)?;

        let room_map_raw: UMap8 = simulate(&options, &initial_planet_map, &depth_field);

        // room_map_raw.debug_print_pretty();

        // let subbed_room_map = sub(&room_map_raw, &map, &depth_field, options.crust_thickness);

        // subbed_room_map.debug_print_pretty();

        // let mut tile_map = TileMap::from_uMap(&subbed_room_map);

        let mut tile_map = TileMap::rooms_planet_combiner(&initial_planet_map, &room_map_raw);

        let rooms = get_rooms(&mut tile_map);

        // tile_map.debug_print();

        let map = sub(
            &room_map_raw,
            &initial_planet_map,
            &depth_field,
            1. - options.crust_thickness,
        );

        let mut image = umap_to_image_buffer(&map);

        image = apply_blur(&image, options.blur);

        let c = march_squares_rgba(&image)?;

        let mut maps: PlanetMap = PlanetMap::empty(options.resolution as usize);
        maps.main = Some(map);
        maps.altitude = Some(altitude_field);
        maps.depth = Some(depth_field);
        maps.rooms_raw = Some(room_map_raw);

        Ok(PlanetData {
            image: Some(image),
            planet_map: maps,
            poly_lines: c,
            tile_map: Some(tile_map),
            rooms: Some(rooms)
        })
    }
}

fn get_rooms(tiles: &mut TileMap) -> Vec<Room> {
    let res = tiles.len();

    let mut room_counter: u16 = 0;
    let mut rooms: Vec<Room> = Vec::new();

    for x in 0..res {
        for y in 0..res {
            match tiles[x][y] {
                Tile::Room(_) => match get_room(x, y, tiles, room_counter, 4) {
                    Some(room) => {
                        tracing::debug!("found room: {:?}", room);
                        rooms.push(room);
                        room_counter += 1;
                    }
                    None => {
                        tracing::debug!("No room found");
                        continue;
                    }
                },
                _ => continue,
            }
            // if tiles[x][y] != Tile::Space {
            //     // println!("found space: {:?} {:?}", x, y);
            //     // continue;
            // } else {
            //     // println!("found candidate: {:?} {:?}", x, y);
            //     match get_room(x, y, tiles, room_counter) {
            //         Some(room) => {
            //             tracing::debug!("found room: {:?}", room);
            //             rooms.push(room);
            //             room_counter += 1;
            //         }
            //         None => {
            //             tracing::debug!("No room found");
            //             continue;
            //         }
            //     }
            // }
        }
    }

    rooms
}

fn get_room(x: usize, y: usize, tile_map: &mut TileMap, id: u16, min_room_size: usize) -> Option<Room> {
    
    let res = tile_map.len() as usize;
    let start_tile = tile_map[x][y];

    if start_tile != Tile::Room(None) {
        return None;
    }

    let mut results: Vec<Coord> = vec![];
    let mut queue = VecDeque::new();

    queue.push_back(Coord { x, y });
    tile_map[x][y] = Tile::Room(Some(id));

    while queue.len() > 0 {
        
        let tile = queue.pop_front().unwrap();
        results.push(tile);

        let this_coord = Coord {
            x: tile.x,
            y: tile.y,
        };

        for c in get_adjacent_coords(&this_coord, res) {
      
            if tile_map[c.x][c.y] != Tile::Room(None) {
                continue;
            }
            tile_map[c.x][c.y] = Tile::Room(Some(id));
            queue.push_back(c);
        }
    }

    if results.len() < min_room_size {
        results.iter()
            .for_each(|c| tile_map[c.x][c.y] = Tile::Wall);
        return None;
    }

    let new_room = Room::new(results, id);

    // println!("center: {:?}", new_room.center);

    tile_map[new_room.center.x][new_room.center.y] = Tile::RoomCenter(id);

    Some(new_room)

}

// fn get_rooms(tiles: &mut TileMap, room_map: &UMap8) -> Vec<Room> {

//     let r = tiles.len();
//     let r2 = room_map[0].len();
//     assert!(r == r2);
//     let mut rooms: Vec<Room> = Vec::new();

//     let mut room_counter: u16 = 0;

//     for x in 0..r {
//         for y in 0..r {
//             if tiles[x][y] != Tile::Space {
//                 // println!("found space: {:?} {:?}", x, y);
//                 // continue;
//             } else {
//                 // println!("found candidate: {:?} {:?}", x, y);
//                 match get_room(x, y, tiles, room_map, room_counter) {
//                     Some(room) => {
//                         tracing::debug!("found room: {:?}", room);
//                         rooms.push(room);
//                         room_counter += 1;
//                     }
//                     None => {
//                         tracing::debug!("No room found");
//                         continue;
//                     }
//                 }
//             }
//         }
//     }

//     rooms
// }

// fn get_room(
//     x: usize,
//     y: usize,
//     tile_map: &mut TileMap,
//     room_raw_map: &UMap8,
//     id: u16,
// ) -> Option<Room> {
//     // tracing::debug!("getting room -----");

//     println!(" starting on {:?} {:?}", x, y);

//     let res = room_raw_map.len() as usize;

//     let source_tile = tile_map[x][y];

//     if source_tile != Tile::Space {
//         return None;
//     }

//     // vec to store this room's tile coords
//     let mut results: Vec<Coord> = vec![];

//     // type to match to
//     let tile_type = tile_map[x][y];

//     // queue to run on
//     let mut queue = VecDeque::new();

//     // init the search queue with one tile
//     // we can be sure that the source tile is valid bc otherwise we would have returned earlier
//     queue.push_back(Coord { x, y });
//     tile_map[x][y] = Tile::Room(Some(id));

//     while queue.len() > 0 {
//         // pop tile to run search on
//         // we can be sure it won't panic bc the while loop would have exited
//         let tile = queue.pop_front().unwrap();

//         // if it's in the queue, we know it's in this room
//         results.push(tile);

//         let this_coord = Coord {
//             x: tile.x,
//             y: tile.y,
//         };

//         // now we search the neighbours and add them to the queue
//         // they will get added to results in a subsequent iteration of the loop

//         for c in get_adjacent_coords(&this_coord, res) {

//             // make sure tile has not been assigned to a room already
//             if tile_map[c.x][c.y] != Tile::Space {
//                 continue;
//             }

//             // or is not of the target type
//             if tile_map[c.x][c.y] != tile_type {
//                 continue;
//             }

//             // mark the tile as visited
//             tile_map[c.x][c.y] = Tile::Room(Some(id));

//             // if we made it this far,
//             queue.push_back(c);
//         }
//     }

//     Some(Room::new(results))
// }

pub fn get_adjacent_coords(coord: &Coord, max_size: usize) -> Vec<Coord> {
    let mut adjacent_coords = Vec::new();

    // Check above
    if coord.y > 0 {
        adjacent_coords.push(Coord {
            x: coord.x,
            y: coord.y - 1,
        });
    }

    // Check below
    if coord.y < max_size - 1 {
        adjacent_coords.push(Coord {
            x: coord.x,
            y: coord.y + 1,
        });
    }

    // Check left
    if coord.x > 0 {
        adjacent_coords.push(Coord {
            x: coord.x - 1,
            y: coord.y,
        });
    }

    // Check right
    if coord.x < max_size - 1 {
        adjacent_coords.push(Coord {
            x: coord.x + 1,
            y: coord.y,
        });
    }

    adjacent_coords
}

fn flatten_and_zip(vertices: &Vec<Vec<Vec2>>) -> Vec<Vec3> {
    vertices
        .iter()
        .flat_map(|digit_points| digit_points.windows(2).flat_map(|window| window))
        .map(|v| Vec3::new(v.x, v.y, 0.0))
        .collect()
}

fn sub(this: &Vec<Vec<u8>>, from: &Vec<Vec<u8>>, mask: &FMap, thresh: f32) -> Vec<Vec<u8>> {
    from.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| {
                    if mask[y][x] > thresh {
                        return val;
                    }

                    let inverted = if this[y][x] == 1 { 0 } else { 1 };
                    val * inverted
                })
                .collect()
        })
        .collect()
}

fn mult(this: &Vec<Vec<u8>>, from: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    from.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| val * this[y][x])
                .collect()
        })
        .collect()
}

// impl FromUMap for TileMap {
//     fn from_uMap16(from: UMap16) -> TileMap {
//         from.iter()
//             .map(|row| {
//                 row.iter()
//                     .map(|entry| if *entry == 1 { Tile::Wall } else { Tile::Space })
//                     .collect()
//             })
//             .collect()
//     }

//     fn from_uMap8(from: UMap8) -> TileMap {
//         from.iter()
//             .map(|row| {
//                 row.iter()
//                     .map(|entry| if *entry == 1 { Tile::Wall } else { Tile::Space })
//                     .collect()
//             })
//             .collect()
//     }
// }

// impl TileMap {
//     fn new(resolution: usize) -> Self {
//         TileMap {
//             tiles: vec![vec![Tile::Space; resolution]; resolution],
//         }
//     }

//     fn from_uMap(uMap: &Vec<Vec<u8>>) -> Self {
//         let mut tiles: Vec<Vec<Tile>> = Vec::new();

//         // for (x,i) in uMap.iter().enumerate(){
//         //     for (y,j) in i.iter().enumerate(){
//         //         if *j == 1{

//         //             tiles[x][y] = Tile::Wall
//         //         }
//         //         else{
//         //             tiles[x][y] = Tile::Space
//         //         }
//         //     }
//         // }

//         let tiles: Vec<Vec<Tile>> = uMap
//             .iter()
//             .map(|row| {
//                 row.iter()
//                     .map(|&entry| if entry == 1 { Tile::Wall } else { Tile::Space })
//                     .collect()
//             })
//             .collect();

//         TileMap { tiles }
//     }
// }

// pub type UMap8 = Vec<Vec<u8>>;
// pub type UMap16 = Vec<Vec<u16>>;

// trait Subtract<Rhs = Self, Output = Self> {
//     fn subtract(&self, other: &Rhs) -> Output;
// }

// impl Subtract for UMap16 {
//     fn subtract(&self, other: &Self) -> Self {
//         self.iter()
//             .zip(other.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(s, o)| (s - o).max(0))
//                     .collect()
//             })
//             .collect()
//     }
// }

// impl Subtract<UMap16, UMap8> for UMap8 {
//     fn subtract(&self, other: &UMap16) -> UMap8 {
//         self.iter()
//             .zip(other.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(s, o)| ((*s as u16) - o).max(0) as u8)
//                     .collect()
//             })
//             .collect()
//     }
// }

// impl Subtract<UMap8, UMap16> for UMap16 {
//     fn subtract(&self, other: &UMap8) -> UMap16 {
//         self.iter()
//             .zip(other.iter())
//             .map(|(v1, v2)| {
//                 v1.iter()
//                     .zip(v2.iter())
//                     .map(|(s, o)| (s - (*o as u16)).max(0))
//                     .collect()
//             })
//             .collect()
//     }
// }
