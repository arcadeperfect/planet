use anyhow::{anyhow, Result};
use delaunator::Triangulation;
use petgraph::{
    algo::min_spanning_tree,
    data::Element,
    graph::{NodeIndex, UnGraph},
};

use crate::{room::Room, roooms::Roooms, types::Coord};

// triangulation: Triangulation,
//     mst: Vec<Element<usize, f32>>,

pub struct RoomTriangulation {
    rooms: Vec<Room>,
}

impl RoomTriangulation {
    pub fn new(rooms: Vec<Room>) -> RoomTriangulation {
        RoomTriangulation { rooms }
    }
}

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

pub fn _calculate_mst(
    triangle_edge_indices: &Vec<(usize, usize)>,
    room_centers: &Vec<Coord>,
) -> Vec<Element<usize, f32>> {
    let mut graph = UnGraph::<usize, f32>::new_undirected();
    let mut node_indexes: Vec<NodeIndex> = Vec::new();

    room_centers.iter().enumerate().for_each(|(i, _room)| {
        node_indexes.push(graph.add_node(i));
    });

    triangle_edge_indices
        .iter()
        .enumerate()
        .for_each(|(_, edge)| {
            ({
                let cost = dist_squared(&room_centers[edge.0], &room_centers[edge.1]);
                graph.add_edge(node_indexes[edge.0], node_indexes[edge.1], cost);
            })
        });

    let mst = min_spanning_tree(&graph);
    let mst_edges: Vec<Element<usize, f32>> = mst.collect();

    mst_edges
}

pub fn get_triangle_edge_indeces(tr: &Triangulation) -> Vec<(usize, usize)> {
    let out: Vec<(usize, usize)> = tr
        .triangles
        .chunks(3)
        .flat_map(|chunk| {
            vec![
                (chunk[0], chunk[1]),
                (chunk[1], chunk[2]),
                (chunk[2], chunk[0]),
            ]
        })
        .collect();

    out
}

pub fn find_mst_indexes(tr: &Triangulation, rooms: &Vec<Room>) -> Vec<(usize, usize)> {
    let room_centers = rooms.iter().map(|room| room.center).collect::<Vec<_>>();
    let edge_indices = get_triangle_edge_indeces(&tr);
    let mst = _calculate_mst(&edge_indices, &room_centers);
    let mut out: Vec<(usize, usize)> = Vec::new();

    for edge in &mst {
        if let Element::Edge { source, target, .. } = edge {
            out.push((*source, *target));
        }
    }

    out
}

pub fn mst_to_coords(rooms: &Vec<Room>, tr: &Triangulation) -> Vec<(Coord, Coord)> {
    
    let mut out: Vec<(Coord, Coord)> = Vec::new();

        let room_centers = rooms.iter().map(|room| room.center).collect::<Vec<_>>();
        let edge_indeces = get_triangle_edge_indeces(&tr);
        
        
        let mst = _calculate_mst(&edge_indeces, &room_centers);

        for edge in mst {
            match edge {
                Element::Edge {
                    source,
                    target,
                    weight: _,
                } => {
                    let a = rooms[source].center;
                    let b = rooms[target].center;
                    out.push((a, b))
                }
                _ => {} // we are not interested in the Node varient of the Enum
            }
        }

    out
}

pub fn triangulation_to_coords(tr: &Triangulation, rooms: &Vec<Room>) -> Vec<(Coord, Coord)> {
    let edge_indeces = get_triangle_edge_indeces(&tr);
    let mut out: Vec<(Coord, Coord)> = vec![];

    for e in edge_indeces {
        let a = rooms[e.0].center;
        let b = rooms[e.1].center;
        out.push((a, b))
    }

    out
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
