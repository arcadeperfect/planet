use delaunator::Triangulation;
use anyhow::{anyhow, Result};

use crate::room::Room;

pub fn delaunate_rooms(rooms: &Vec<Room>) -> Result<Triangulation> {
    let points: Vec<delaunator::Point> = rooms
        .iter()
        .map(|room| delaunator::Point {
            x: (room.tiles[0].x as f64),
            y: (room.tiles[0].y as f64),
        })
        .collect();

    let t = delaunator::triangulate(&points);

    if t.is_empty() {
        return Err(anyhow!("triangulation failed"));
    }
    tracing::info!("triangulation succeeded with {:?} nodes", t.len());

    Ok(t)
}