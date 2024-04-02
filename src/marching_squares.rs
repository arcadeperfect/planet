use anyhow::{anyhow, Result};

use image::RgbaImage;
use marching_squares::{Field, Line, Point};

use crate::{
    traits::FromMarchingSquareLine,
    types::{IMap16, PolyLines, UMap16},
};

#[allow(dead_code)]
pub fn march_squares_umap(map: &UMap16) -> Result<PolyLines> {
    if map.len() == 0 {
        return Err(anyhow!("0 length data passed to gett"));
    }

    let p = map
        .iter()
        .map(|x| x.iter().map(|y| *y as i16 * 255).collect::<Vec<i16>>())
        .collect::<Vec<Vec<i16>>>();

    Ok(get_contours(p as Vec<Vec<i16>>)?)
}

pub fn march_squares_rgba(rgba: &RgbaImage) -> Result<PolyLines> {
    if rgba.width() == 0 || rgba.height() == 0 {
        return Err(anyhow!("0 length data passed to march_squares_rgba"));
    }

    // Convert the image to grayscale and scale the values
    let p = rgba
        .pixels()
        .map(|pixel| {
            // Convert to grayscale using a simple average. You might want to use a different formula.
            let gray = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;
            // Scale the value
            gray as i16
        })
        .collect::<Vec<i16>>();

    // Since get_contours expects a Vec<Vec<i16>>, we need to reconstruct the 2D structure
    let width = rgba.width() as usize;
    let height = rgba.height() as usize;
    let p_2d: IMap16 = (0..height)
        .map(|y| p[(y) * width..(y + 1) * width].to_vec())
        .collect::<Vec<Vec<i16>>>();

    Ok(get_contours(p_2d)?)
}

fn get_contours(input: Vec<Vec<i16>>) -> Result<PolyLines> {
    if input.len() == 0 {
        return Err(anyhow!("0 length data passed to get_contours"));
    }

    println!("getting contours");

    let w = input[0].len();
    let h = input.len();
    let thresh = 125;

    let field = Field {
        dimensions: (w, h),
        top_left: Point { x: 0., y: 0. },
        pixel_size: (1., 1.),
        values: &input,
    };

    let mut f: Vec<Line> = field.get_contours(thresh);

    if f.len() == 0 {
        return Err(anyhow!("no contours found"));
    }

    for line in &mut f {
        for point in &mut line.points {
            point.x = point.x / w as f32 * 2. - 1.;
            point.y = -(point.y / h as f32 * 2. - 1.);
        }
    }

    let out = f
        .iter()
        .map(|v| Vec::from_marching_square_line(v))
        .collect();

    Ok(out)
}
