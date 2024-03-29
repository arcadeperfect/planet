use crate::{
    room::Room,
    tile_map::{Tile, TileMap},
    triangulation::{delaunate_rooms, mst_indexes_by_index, triangulation_to_coords},
    types::Coord,
};
use anyhow::Result;
use delaunator::Triangulation;

#[derive(Debug, Clone)]
pub struct Roooms {
    /// the room structs
    pub rooms: Vec<Room>,
    triangulation: Option<Triangulation>,
    /// the mininum spanning tree represented as index pairs of the rooms
    pub mst: Option<Vec<(usize, usize)>>,
}

impl Roooms {
    pub fn new(tiles: &mut TileMap) -> Result<Roooms> {
        println!(" new rooooms");

        let res = tiles.len();
        let mut room_counter: u16 = 0;
        let mut mrooms: Vec<Room> = Vec::new();

        for x in 0..res {
            for y in 0..res {
                match tiles[x][y] {
                    Tile::Room(_) => match Room::generate_room((x, y), tiles, room_counter, 15) {
                        Some(room) => {
                            tracing::debug!("found room: {:?}", room);
                            mrooms.push(room);
                            room_counter += 1;
                        }
                        None => {
                            tracing::debug!("No valid room found");
                            continue;
                        }
                    },
                    _ => continue,
                }
            }
        }
        let rooms = mrooms;
        let tri = delaunate_rooms(&rooms).ok();
        let mut mst = None;

        if let Some(tr) = &tri {
            mst = Some(mst_indexes_by_index(&tr, &rooms));
        }
        Ok(Roooms {
            rooms,
            triangulation: tri,
            mst,
        })
    }

    pub fn get_room_centers(&self) -> Vec<Coord> {
        self.rooms.iter().map(|room| room.center).collect()
    }

    pub fn get_mst_as_coord(&self) -> Vec<(Coord, Coord)> {
        if let Some(mst) = &self.mst {
            return mst
                .iter()
                .map(|(a, b)| (self.rooms[*a].center, self.rooms[*b].center))
                .collect();
        }
        vec![]
    }

    pub fn get_triangulation_coords(&self) -> Option<Vec<(Coord, Coord)>> {
        if let Some(tr) = &self.triangulation {
            Some(triangulation_to_coords(tr, &self.rooms))
        } else {
            None
        }
    }
}
