pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,//position in world
    pub yaw: f32,//left right rotation
    pub cam_z: f32,//distance if eye to screen
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,  
            yaw: 0.0,
            cam_z: 500.0,
        }
    }
}