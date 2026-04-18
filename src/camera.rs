pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: -400.0,  
        }
    }
}