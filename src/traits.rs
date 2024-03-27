use delaunator::Point as DelaunatorPoint;
use glam::Vec2;
use marching_squares::Line as MarchingQuaresLine;
use marching_squares::Point as MarchingSquarePoint;

pub trait FromDelaunatorPoint {
    fn from_delaunator_point(p: DelaunatorPoint) -> Vec2;
}

impl FromDelaunatorPoint for Vec2 {
    fn from_delaunator_point(p: DelaunatorPoint) -> Vec2 {
        Vec2 {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

pub trait FromMarchingSquarePoint {
    fn from_marching_square_point(point: MarchingSquarePoint) -> Vec2;
}

impl FromMarchingSquarePoint for Vec2 {
    fn from_marching_square_point(point: MarchingSquarePoint) -> Vec2 {
        Vec2 {
            x: point.x as f32,
            y: point.y as f32,
        }
    }
}

pub trait FromMarchingSquareLine {
    fn from_marching_square_line(line: &MarchingQuaresLine) -> Vec<Vec2>;
}

impl FromMarchingSquareLine for Vec<Vec2> {
    fn from_marching_square_line(line: &MarchingQuaresLine) -> Vec<Vec2> {
        line.points
            .iter()
            .map(|p| Vec2::from_marching_square_point(*p))
            .collect()
    }
}
