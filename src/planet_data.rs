use delaunator::Triangulation;
use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgba};
use petgraph::data::Element;

use crate::{
    room::Room, roooms::Roooms, tile_map::TileMap, triangulation::{_calculate_mst, get_triangle_edge_indeces}, types::{Coord, PlanetMap}
};

#[derive(Clone, Debug)]
pub struct PlanetData {
    pub image: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub planet_map: PlanetMap,
    pub poly_lines: Vec<Vec<Vec2>>,
    pub tile_map: Option<TileMap>,
    // pub rooms: Option<Vec<Room>>,
    // pub triangulation: Option<Triangulation>,
    pub mst: Option<Vec<(Coord, Coord)>>,
    pub roooms: Option<Roooms>,
}

impl PlanetData {
    /// return the poly lines as a flattened list where each pair represents a line segment
    /// this results in a lot of doubled points, but this is how the shader likes it
    pub fn get_line_list(&self) -> Vec<Vec3> {
        flatten_and_zip(&self.poly_lines)
    }

    pub fn get_dimension(&self) -> Option<usize> {
        match &self.image {
            Some(_) => Some(self.tile_map.as_ref().unwrap().len()),
            None => None,
        }
    }

    // pub fn get_centers(&self) -> Option<Vec<Coord>> {
    //     match &self.rooms {
    //         Some(rooms) => Some(rooms.iter().map(|room| room.center).collect()),
    //         None => None,
    //     }
    // }
    

    // pub fn get_mst(&self) -> Vec<(Coord, Coord)> {

    //     if let Some(tr) = &self.triangulation{
    //     let t = get_triangle_edge_indeces(&tr);

    //         let r = self.get_centers().unwrap();

    //         let m = mst(&t, &r);

    //         let mut out: Vec<(Coord, Coord)> = Vec::new();

    //         for edge in m {
    //             match edge {
    //                 Element::Edge {
    //                     source,
    //                     target,
    //                     weight: _,
    //                 } => {
    //                     if let Some(rooms) = self.rooms.as_ref() {
    //                         let a = rooms[source].center;
    //                         let b = rooms[target].center;
    //                         out.push((a, b))
    //                     }
    //                 }
    //                 _ => {} // we are not interested in the Node varient of the Enum
    //             }
    //         }
    //         out
    //     }
    //     else{
    //         vec![]
    //     }
    // }

    // pub fn get_triangle_tripple_indeces(&self) -> Option<Vec<(usize, usize, usize)>> {
    //     if let Some(tr) = &self.triangulation {
    //         let out: Vec<(usize, usize, usize)> = tr
    //             .triangles
    //             .chunks(3)
    //             .map(|chunk| (chunk[0], chunk[1], chunk[2]))
    //             .collect();

    //         return Some(out);

    //         // let out: Vec<(usize, usize)> = tr.triangles
    //         // .chunks(3)
    //         // .flat_map(|chunk| {
    //         //     vec![
    //         //         (chunk[0], chunk[1]),
    //         //         (chunk[1], chunk[2]),
    //         //         (chunk[2], chunk[0]),
    //         //     ]
    //         // })
    //         // .collect();
    //     }
    //     None
    // }

    // pub fn get_triangle_edge_indeces(&self) -> Option<Vec<(usize, usize)>> {
    //     if let Some(tr) = &self.triangulation {
    //         let out: Vec<(usize, usize)> = tr
    //             .triangles
    //             .chunks(3)
    //             .flat_map(|chunk| {
    //                 vec![
    //                     (chunk[0], chunk[1]),
    //                     (chunk[1], chunk[2]),
    //                     (chunk[2], chunk[0]),
    //                 ]
    //             })
    //             .collect();
    //         return Some(out);
    //     }
    //     None
    // }

    // pub fn get_triangle_edges(&self) -> Option<Vec<(Coord, Coord)>> {
    //     let v = self.get_triangle_edge_indeces();
    //     if let Some(v) = v {
    //         let mut out = Vec::new();
    //         for (a, b) in v {
    //             out.push((
    //                 self.get_centers().unwrap()[a],
    //                 self.get_centers().unwrap()[b],
    //             ));
    //         }
    //         return Some(out);
    //     }

    //     None
    // }
}

fn flatten_and_zip(vertices: &Vec<Vec<Vec2>>) -> Vec<Vec3> {
    vertices
        .iter()
        .flat_map(|digit_points| digit_points.windows(2).flat_map(|window| window))
        .map(|v| Vec3::new(v.x, v.y, 0.0))
        .collect()
}

