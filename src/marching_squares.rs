use std::time::Instant;

use glam::Vec2;
use marching_squares::{Field, Line, Point};
use anyhow::{anyhow, Result};

use crate::{traits::FromMarchingSquareLine, types::{PlanetMap, PolyLines, UMap}};

pub fn march_squares(map: &UMap) -> Result<PolyLines>{
    
    if map.len() == 0 {
        return Err(anyhow!("0 length data passed to gett"))
    }


    let p = map.iter().map(|x| x.iter().map(|y| *y as i16 * 255).collect::<Vec<i16>>()).collect::<Vec<Vec<i16>>>();
    
    Ok(get_contours(p as Vec<Vec<i16>>)?)

}



fn get_contours(input: Vec<Vec<i16>>) -> Result<PolyLines>{

    if input.len() == 0 {
        return Err(anyhow!("0 length data passed to get_contours"))
    }


    let w = input[0].len();
    let h = input.len();
    let thresh = 125;

    let field = Field{
        dimensions: (w, h),
        top_left: Point{x:0., y:0.},
        pixel_size: (1., 1.),
        values: &input,
    };

    let instant = Instant::now();
    let mut f: Vec<Line> = field.get_contours(thresh);

    if f.len() == 0 {
        return Err(anyhow!("no contours found"))
    }

    println!("contours returned {}", f.len());

    println!("get contours took {:?}", instant.elapsed());

    for line in &mut f{
        for point in &mut line.points{
            point.x = point.x / w as f32 * 2.-1.;
            point.y = point.y / h as f32 * 2.-1.;
        }
    }

    let out = f.iter().map(|v| Vec::from_marching_square_line(v)).collect();

    Ok(out)
}