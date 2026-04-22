use crate::input::InputState;
use crate::math::Vertex;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,        // position in world
    pub yaw: f32,      // rotation around Y axis (look left/right)
    pub pitch: f32,    // rotation around X axis (look up/down)
    pub cam_z: f32,    // focal length: distance from eye to projection plane
    move_speed: f32,
    turn_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            cam_z: 500.0,
            move_speed: 200.0,
            turn_speed: 1.5,
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32) {
        let (sin_y, cos_y) = self.yaw.sin_cos();

        // Move along the camera's facing direction so W/S go where you're looking.
        // Pitch is intentionally ignored for movement — you don't fly when you look up.
        let forward = (sin_y, cos_y);
        let strafe  = (cos_y, -sin_y);

        if input.w { self.x += forward.0 * self.move_speed * dt; self.z += forward.1 * self.move_speed * dt; }
        if input.s { self.x -= forward.0 * self.move_speed * dt; self.z -= forward.1 * self.move_speed * dt; }
        if input.d { self.x += strafe.0  * self.move_speed * dt; self.z += strafe.1  * self.move_speed * dt; }
        if input.a { self.x -= strafe.0  * self.move_speed * dt; self.z -= strafe.1  * self.move_speed * dt; }

        // Screen Y points down, so "up" decreases y, "down" increases it.
        if input.up   { self.y -= self.move_speed * dt; }
        if input.down { self.y += self.move_speed * dt; }

        if input.left  { self.yaw -= self.turn_speed * dt; }
        if input.right { self.yaw += self.turn_speed * dt; }

        // Clamp pitch so we never look past straight up/down (avoids gimbal flip).
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.05;
        if input.pitch_up   { self.pitch += self.turn_speed * dt; }
        if input.pitch_down { self.pitch -= self.turn_speed * dt; }
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
    }

    /// Project a world-space vertex to screen pixel coordinates.
    /// Returns None when the point sits behind the projection plane.
    pub fn project(&self, v: Vertex, screen_w: f32, screen_h: f32) -> Option<(f32, f32)> {
        let cx = screen_w * 0.5;
        let cy = screen_h * 0.5;

        // Translate so the camera sits at the screen center.
        let rx = v.x - self.x - cx;
        let ry = v.y - self.y - cy;
        let rz = v.z - self.z;

        // View rotation: Rx(-pitch) * Ry(-yaw) — yaw first (around world Y),
        // then pitch around the camera's local right axis.
        let (sin_y, cos_y) = (-self.yaw).sin_cos();
        let yx =  rx * cos_y + rz * sin_y;
        let yy =  ry;
        let yz = -rx * sin_y + rz * cos_y;

        let (sin_p, cos_p) = (-self.pitch).sin_cos();
        let view_x = yx;
        let view_y = yy * cos_p - yz * sin_p;
        let view_z = yy * sin_p + yz * cos_p;

        let denom = view_z + self.cam_z;
        if denom <= 0.1 { return None; }
        let factor = self.cam_z / denom;

        Some((view_x * factor + cx, view_y * factor + cy))
    }
}
