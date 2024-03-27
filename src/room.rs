use crate::types::Coord;
use std::collections::HashSet;

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
        get_center(tiles, edges)
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

        // for c in &self.tiles {
        //     println!("{} {}", c.x, c.y);
        // }

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

fn get_center(tiles: &[Coord], edges: &[usize]) -> Coord {
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
            // center.x = tile.y;
            // center.y = tile.x;
        }
    }
    center
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
