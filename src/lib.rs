#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32,z:f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_point(self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn translate(self, dx: Option<f32>, dy: Option<f32>, dz: Option<f32>) -> Self {
        Self {
            x: self.x + dx.unwrap_or(0.0),
            y: self.y + dy.unwrap_or(0.0),
            z: self.z + dz.unwrap_or(0.0),
        }
    }

    pub fn scale(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}