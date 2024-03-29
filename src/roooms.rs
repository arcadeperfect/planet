use crate::{
    room::Room,
    tile_map::{Tile, TileMap},
    triangulation::{delaunate_rooms, find_mst_indexes},
    types::Coord,
};
use anyhow::Result;
use delaunator::Triangulation;


#[derive(Debug, Clone)]
pub struct Roooms {
    pub rooms: Vec<Room>,
    triangulation: Option<Triangulation>,
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
            mst = Some(find_mst_indexes(&tr, &rooms));
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

}

fn dist_squared(a: &Coord, b: &Coord) -> f32 {
    let dx = a.x as i32 - b.x as i32;
    let dy = a.y as i32 - b.y as i32;
    (dx * dx + dy * dy) as f32
}

fn dist(a: &Coord, b: &Coord) -> f32 {
    let dx = a.x as i32 - b.x as i32;
    let dy = a.y as i32 - b.y as i32;
    ((dx * dx + dy * dy) as f32).sqrt()
}
