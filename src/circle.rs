use crate::math::{rotation_xyz, PROJECTION};
use crate::vertex::Vertex;

pub struct Circle {
    center: Vertex,
    radius: f32,
    rotation_angle: f32,
}

impl Circle {
    pub fn new(center: Vertex, radius: f32) -> Self {
        Self { center, radius, rotation_angle: 0.0 }
    }
}
