use std::collections::HashSet;

use uuid::Uuid;

use crate::types::Coord;

#[derive(Clone, Default, Debug)]
pub struct Room {
    pub tiles: Vec<Coord>,
    pub tiles_hash: HashSet<Coord>,
    pub center: Coord,
    pub edge_tile_indexes: Vec<usize>,
    pub id: Uuid,
}

impl Room {
    pub fn new(tiles: Vec<Coord>) -> Self {
        let tiles_hash = tiles.iter().cloned().collect();
        let edge_tile_indexes = Room::find_edges(&tiles, &tiles_hash);
        let center = Room::calc_center(&tiles, &edge_tile_indexes);
        let id = uuid::Uuid::new_v4();
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

    let mut center = Coord::default(); // Ensure this has a sensible default
    let mut max_min_d = f32::MIN;

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
