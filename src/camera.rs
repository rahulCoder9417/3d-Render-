
use crate::input::InputState;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,//position in world
    pub yaw: f32,//left right rotation
    pub cam_z: f32,//distance if eye to screen
    move_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,  
            yaw: 0.0,
            cam_z: 500.0,
            move_speed: 200.0,
        }
    }
    
    pub fn update(&mut self,input:&InputState, dt: f32) {
       if input.w { self.z += self.move_speed * dt; }
        if input.s { self.z -= self.move_speed * dt; }
        if input.a { self.x -= self.move_speed * dt; }
        if input.d { self.x += self.move_speed * dt; }
        if input.left  { self.yaw -= 1.5 * dt; }
        if input.right { self.yaw += 1.5 * dt; }
    }
}