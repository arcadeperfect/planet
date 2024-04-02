use crate::{
    bit_map::{average_center, edge_average_center, max_inscribed_circle}, debug_print::TileMapDebug, tile_map::{Status, Tile, TileMap}, types::Coord
};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Default, Debug)]
pub struct Room {
    pub tiles: Vec<Coord>,
    pub tiles_hash: HashSet<Coord>,
    pub center: Coord,
    pub edge_tile_indexes: Vec<usize>,
    pub id: u16,
}

impl Room {
    pub fn new(tiles: Vec<Coord>, id: u16) -> Self {
        let tiles_hash = tiles.iter().cloned().collect();
        let edge_tile_indexes = Room::find_edges(&tiles, &tiles_hash);
        let center = Room::calc_center(&tiles, &edge_tile_indexes);

        Room {
            tiles,
            tiles_hash,
            center,
            edge_tile_indexes,
            id,
        }
    }

    /// Attempts to generate a room
    ///
    /// Uses floodfill algorithm to generate a room, starting on the specified
    /// tile. If succesful, mutates the tile map and returns a room struct.
    pub fn generate_room(
        search_start: (usize, usize),
        tile_map: &mut TileMap,
        id: u16,
        min_room_size: usize,
    ) -> Option<Room> {

        
        

        let x = search_start.0;
        let y = search_start.1;

        let res = tile_map.len() as usize;
        let start_tile = tile_map[x][y];

        if start_tile != Tile::Room(Status::Undesignated) {
            return None;
        }

        let mut results: Vec<Coord> = vec![];
        let mut queue = VecDeque::new();

        queue.push_back(Coord { x, y });
        tile_map[x][y] = Tile::Room(Status::Designated(id));

        while queue.len() > 0 {
            let tile = queue.pop_front().unwrap();
            results.push(tile);

            let this_coord = Coord {
                x: tile.x,
                y: tile.y,
            };

            for adjacent_coord in get_adjacent_coords(&this_coord, res) {
                if tile_map[adjacent_coord.x][adjacent_coord.y] != Tile::Room(Status::Undesignated) {
                    continue;
                }
                tile_map[adjacent_coord.x][adjacent_coord.y] = Tile::Room(Status::Designated(id));
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

    pub fn get_edge_tiles(&self) -> Vec<Coord> {
        let edge_tiles: Vec<Coord> = self
            .edge_tile_indexes
            .iter()
            .map(|i| self.tiles[*i])
            .collect();
        edge_tiles
    }

    fn find_edges(tiles: &Vec<Coord>, hash: &HashSet<Coord>) -> Vec<usize> {
        get_edge_indexes(tiles, hash)
    }

    fn calc_center(tiles: &Vec<Coord>, edges: &Vec<usize>) -> Coord {
        // max_inscribed_circle(tiles, edges)
        edge_average_center(tiles, edges)
        // get_center(tiles, edges)
    }

    pub fn get_min_max_coords(&self) -> (Coord, Coord) {
        let mut max = Coord::min();
        let mut min = Coord::max();
        for tile in &self.tiles {
            max.x = max.x.max(tile.x);
            max.y = max.y.max(tile.y);
            min.x = min.x.min(tile.x);
            min.y = min.y.min(tile.y);
        }
        (min, max)
    }

    pub fn debug_print(&self) {
        println!("Room id: {}", self.id);

        let (min, max) = self.get_min_max_coords();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let coord = Coord { x, y };

                if self.center == coord {
                    print!("X ");
                } else if self.tiles_hash.contains(&coord) {
                    print!("o ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }

        println!("Min: {} {} Max: {} {}", min.x, min.y, max.x, max.y);
    }
}

fn get_edge_indexes(tiles: &[Coord], hash: &HashSet<Coord>) -> Vec<usize> {
    tiles
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if get_neighbouring_coords_vertical_horizontal(c)
                .iter()
                .any(|n| !hash.contains(n))
            {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn get_neighbouring_coords_vertical_horizontal(c: &Coord) -> Vec<Coord> {
    vec![*c + (1, 0), *c + (0, 1), *c + (0, -1), *c + (-1, 0)]
}

fn get_neighbouring_coords_all(c: &Coord) -> Vec<Coord> {
    vec![
        *c + (1, 0),
        *c + (1, 1),
        *c + (0, 1),
        *c + (-1, 1),
        *c + (0, -1),
        *c + (-1, -1),
        *c + (-1, 0),
        *c + (1, -1),
    ]
}

fn get_neighbouring_coords_diagonal(c: &Coord) -> Vec<Coord> {
    vec![*c + (1, 1), *c + (-1, 1), *c + (-1, -1), *c + (1, -1)]
}

// fn get_center(tiles: &[Coord], edges: &[usize]) -> Coord {
//     let edges_hash: HashSet<Coord> = edges.iter().map(|&i| tiles[i]).collect();
//     let mut center = Coord::default();
//     let mut max_min_d = f32::MIN;

//     if edges_hash.len() == tiles.len() {
//         let sum = tiles
//             .iter()
//             .fold((0, 0), |acc, coord| (acc.0 + coord.x, acc.1 + coord.y));
//         let count = tiles.len();
//         return Coord {
//             x: sum.0 / count,
//             y: sum.1 / count,
//         };
//     }

//     for &tile in tiles.iter().filter(|&&t| !edges_hash.contains(&t)) {
//         let min_d = edges
//             .iter()
//             .filter_map(|&i| {
//                 let distance = dist_squared(&tile, &tiles[i]);
//                 if distance.is_nan() {
//                     None
//                 } else {
//                     Some(distance)
//                 }
//             })
//             .fold(f32::INFINITY, f32::min); // We want the minimum distance to the edges

//         if min_d > max_min_d {
//             max_min_d = min_d;
//             center = tile;
//             // center.x = tile.y;
//             // center.y = tile.x;
//         }
//     }
//     center
// }

pub fn generate_rooms(tiles: &mut TileMap) -> Vec<Room> {
    let res = tiles.len();
    let mut room_counter: u16 = 0;
    let mut rooms: Vec<Room> = Vec::new();

    tiles.debug_print();

    for x in 0..res {
        for y in 0..res {
            match tiles[x][y] {
                Tile::Room(_) => match Room::generate_room((x, y), tiles, room_counter, 15) {
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

// fn get_room(
//     x: usize,
//     y: usize,
//     tile_map: &mut TileMap,
//     id: u16,
//     min_room_size: usize,
// ) -> Option<Room> {

//     let res = tile_map.len() as usize;
//     let start_tile = tile_map[x][y];

//     if start_tile != Tile::Room(None) {
//         return None;
//     }

//     let mut results: Vec<Coord> = vec![];
//     let mut queue = VecDeque::new();

//     queue.push_back(Coord { x, y });
//     tile_map[x][y] = Tile::Room(Some(id));

//     while queue.len() > 0 {
//         let tile = queue.pop_front().unwrap();
//         results.push(tile);

//         let this_coord = Coord {
//             x: tile.x,
//             y: tile.y,
//         };

//         for adjacent_coord in get_adjacent_coords(&this_coord, res) {
//             if tile_map[adjacent_coord.x][adjacent_coord.y] != Tile::Room(None) {
//                 continue;
//             }
//             tile_map[adjacent_coord.x][adjacent_coord.y] = Tile::Room(Some(id));
//             queue.push_back(adjacent_coord);
//         }
//     }

//     // erase if below min size
//     if results.len() < min_room_size {
//         results.iter().for_each(|c| tile_map[c.x][c.y] = Tile::Wall);
//         return None;
//     }

//     let new_room = Room::new(results, id);

//     tile_map[new_room.center.x][new_room.center.y] = Tile::RoomCenter(id);

//     for edge_tile_index in &new_room.edge_tile_indexes {
//         let e = new_room.tiles[*edge_tile_index];
//         tile_map[e.x][e.y] = Tile::RoomEdge(id);
//     }

//     Some(new_room)
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

fn dist_squared(a: &Coord, b: &Coord) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy) as f32
}

fn dist(a: &Coord, b: &Coord) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    ((dx * dx + dy * dy) as f32).sqrt()
}

// pub fn nearest_neighbor(a: &Vec<Coord>, b: &Vec<Coord>) -> (Coord, Coord){
//     let mut dist = f32::MAX;
//     let mut s_a = a[0];
//     let mut s_b = b[0];
//     for c_a in &a{
//         for c_b in &b{
//             let d = dist_squared(&c_a, &c_b);
//             if d < dist {
//                 dist = d;
//                 s_a = *c_a;
//                 s_b = *c_b;
//             }
//         }
//     }
//     (s_a, s_b)
// }




pub fn closest_tiles(a: &Room, b: &Room) -> (Coord, Coord) {

    let e1 = &a.edge_tile_indexes;
    let e2 = &b.edge_tile_indexes;

    let mut dist = f32::MAX;
    let mut s_a: usize = 0;
    let mut s_b: usize = 0;

    for y in e1{
        for z in e2{
            let y_tile = a.tiles[*y];
            let z_tile = b.tiles[*z];
            let d = dist_squared(&y_tile, &z_tile);
            if d < dist {
                dist = d;
                s_a = *y;
                s_b = *z;
            }
        }

    }

    (a.tiles[s_a], b.tiles[s_b])

}

