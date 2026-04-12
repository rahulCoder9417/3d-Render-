use crate::math::{Col3, Mat3, mat3_mul_col};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_col(self) -> Col3 {
        [self.x, self.y, self.z]
    }

    pub fn from_col(col: Col3) -> Self {
        Self { x: col[0], y: col[1], z: col[2] }
    }

    pub fn to_screen(self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn translate(self, dx: f32, dy: f32, dz: f32) -> Self {
        Self { x: self.x + dx, y: self.y + dy, z: self.z + dz }
    }

    pub fn scale(self, factor: f32) -> Self {
        Self { x: self.x * factor, y: self.y * factor, z: self.z * factor }
    }

    /// Apply a 3x3 matrix transform (rotation, projection, etc.)
    pub fn transform(self, m: &Mat3) -> Self {
        Self::from_col(mat3_mul_col(m, &self.to_col()))
    }
}