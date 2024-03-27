#![allow(dead_code)]

use anyhow::Result;
use bitmap::{apply_blur, get_initial_planet_map, umap_to_image_buffer};
use cellular_automata::simulate;
use glam::{Vec2, Vec3};
use marching_squares::march_squares_rgba;
use noise::permutationtable::PermutationTable;
use room::Room;
use std::collections::VecDeque;
pub use types::PlanetOptions;
use types::{Coord, FMap, FractalNoiseOptions, PlanetData, PlanetMap, UMap8};

use crate::tile_map::{FromUMap, Tile, TileMap};

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
            get_initial_planet_map(&options, fractal_options)?;

        let room_map_raw: UMap8 = simulate(&options, &initial_planet_map, &depth_field);

        // room_map_raw.debug_print_pretty();

        let mut tile_map = TileMap::rooms_planet_combiner(&initial_planet_map, &room_map_raw);
        // tile_map.debug_print();
        
        let room_structs = process_rooms(&mut tile_map);

        

        // room_structs.debug_print();
        // room_structs.iter().for_each(|room|{room.debug_print()});
        // tile_map.debug_print();


        let map_main:UMap8 = sub(
            &room_map_raw,
            &initial_planet_map,
            &depth_field,
            1. - options.crust_thickness,
        );

        // map_main.debug_print_pretty();

        let mut image = umap_to_image_buffer(&map_main);

        image = apply_blur(&image, options.blur);

        let polylines = march_squares_rgba(&image)?;

        let mut maps: PlanetMap = PlanetMap::empty(options.resolution as usize);
        maps.main = Some(map_main);
        maps.altitude = Some(altitude_field);
        maps.depth = Some(depth_field);
        maps.rooms_raw = Some(room_map_raw);

        Ok(PlanetData {
            image: Some(image),
            planet_map: maps,
            poly_lines: polylines,
            tile_map: Some(tile_map),
            rooms: Some(room_structs),
        })
    }
}

fn process_rooms(tiles: &mut TileMap) -> Vec<Room> {
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
        }
    }
    rooms
}

fn get_room(
    x: usize,
    y: usize,
    tile_map: &mut TileMap,
    id: u16,
    min_room_size: usize,
) -> Option<Room> {

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

        for adjacent_coord in get_adjacent_coords(&this_coord, res) {
            if tile_map[adjacent_coord.x][adjacent_coord.y] != Tile::Room(None) {
                continue;
            }
            tile_map[adjacent_coord.x][adjacent_coord.y] = Tile::Room(Some(id));
            queue.push_back(adjacent_coord);
        }
    }

    // erase if below min size
    if results.len() < min_room_size {
        results.iter().for_each(|c| tile_map[c.x][c.y] = Tile::Wall);
        return None;
    }

    let new_room = Room::new(results, id);

    tile_map[new_room.center.x][new_room.center.y] = Tile::RoomCenter(id);

    for edge_tile_index in &new_room.edge_tile_indexes {
        let e = new_room.tiles[*edge_tile_index];
        tile_map[e.x][e.y] = Tile::RoomEdge(id);
    }

    Some(new_room)
}

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

trait DebugPrint {
    fn debug_print(&self);
}

impl DebugPrint for Vec<Room> {
    fn debug_print(&self) {
        println!("");
        println!("Debug print room vec --- {:?} rooms \n", self.len());

        let mut min = Coord::max();
        let mut max = Coord::min();

        for room in self {
            let (room_min, room_max) = room.get_min_max_coords();

            min.x = min.x.min(room_min.x);
            min.y = min.y.min(room_min.y);

            max.x = max.x.max(room_max.x);
            max.y = max.y.max(room_max.y);
        }

        println!("Min: {} {} Max: {} {}", min.x, min.y, max.x, max.y);

        for y in min.y..=max.y{
            for x in min.x..=max.x {
                let c = Coord { x, y };
                if self.iter().any(|r| r.tiles.contains(&c)) {
                    print!("X ");
                } else {
                    print!("  ");
                }
            }
            println!("");
        }

        // for x in min.y..=max.y {
        //     for y in min.x..=max.x {
        //         let c = Coord { x, y };
        //         if self.iter().any(|r| r.tiles.contains(&c)) {
        //             print!("X ");
        //         } else {
        //             print!("  ");
        //         }
        //     }
        //     println!("");
        // }
        
    }
}