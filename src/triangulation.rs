use anyhow::{anyhow, Result};
use delaunator::Triangulation;
use petgraph::{
    algo::min_spanning_tree,
    data::Element,
    graph::{NodeIndex, UnGraph},
};

use crate::{room::Room, types::Coord};

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

// pub fn get_mst_coords(
//     triangulation_edge_indices: &Vec<(usize, usize)>,
//     room_center_coords: &Vec<Coord>,
// ) -> Vec<Coord> {
//     let i = get_mst_indices(triangulation_edge_indices, room_center_coords);
//     let v: Vec<Coord> = i.iter().map(|i| room_center_coords[*i]).collect();
//     v
// }

// pub fn get_mst_indices(
//     triangulation_edge_indices: &Vec<(usize, usize)>,
//     room_nodes: &Vec<Coord>,
// ) -> Vec<usize> {
//     let mst = calculate_mst(triangulation_edge_indices, room_nodes);

//     let mut out: Vec<usize> = Vec::new();

//     for edge in &mst {
//         match edge {
//             Element::Edge {
//                 source,
//                 target: _,
//                 weight: _,
//             } => out.push(*source),
//             _ => {} // we are not interested in the Node varient of the Enum
//         }
//     }

//     if let Some(Element::Edge {
//         source: _,
//         target,
//         weight: _,
//     }) = mst.last()
//     {
//         out.push(*target);
//     }

//     out
// }

// fn calculate_mst(
//     triangulation_edge_indices: &Vec<(usize, usize)>,
//     room_nodes: &Vec<Coord>,
// ) -> Vec<Element<usize, f32>> {
//     let mut graph = UnGraph::<usize, f32>::new_undirected();
//     let mut node_indexes: Vec<NodeIndex> = Vec::new();
//     room_nodes.iter().enumerate().for_each(|(i, _room)| {
//         node_indexes.push(graph.add_node(i));
//     });
//     triangulation_edge_indices
//         .iter()
//         .enumerate()
//         .for_each(|(_, edge)| {
//             ({
//                 let cost = dist(&room_nodes[edge.0], &room_nodes[edge.1]);
//                 graph.add_edge(node_indexes[edge.0], node_indexes[edge.1], cost);
//             })
//         });
//     let mst = min_spanning_tree(&graph);
//     let mst_edges: Vec<Element<usize, f32>> = mst.collect();

//     mst_edges
// }


pub fn mst(triangle_edge_indices: &Vec<(usize, usize)>, room_centers: &Vec<Coord>) -> Vec<Element<usize, f32>> {
    let mut graph = UnGraph::<usize, f32>::new_undirected();

    let mut node_indexes: Vec<NodeIndex> = Vec::new();

    room_centers.iter().enumerate().for_each(|(i, _room)| {
        node_indexes.push(graph.add_node(i));
    });

    triangle_edge_indices.iter().enumerate().for_each(|(_, edge)| {
        ({
            let cost = dist(&room_centers[edge.0], &room_centers[edge.1]);
            graph.add_edge(node_indexes[edge.0], node_indexes[edge.1], cost);
        })
    });

    let mst = min_spanning_tree(&graph);
    let mst_edges: Vec<Element<usize, f32>> = mst.collect();

    mst_edges
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
