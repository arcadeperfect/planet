use std::f32::consts::PI;

use glam::Vec2;

pub fn circle(center: Vec2, radius: f32, num_segments: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(num_segments);

    for i in 0..=num_segments {
        let angle = 2.0 * PI * (i as f32) / (num_segments as f32);
        let x = radius * angle.cos() + center.x;
        let y = radius * angle.sin() + center.y;
        points.push(Vec2::new(x, y));
    }

    points
}