#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_point(self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn translate(self, dx: f32, dy: f32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn scale(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}