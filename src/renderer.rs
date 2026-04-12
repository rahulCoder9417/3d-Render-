use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::cube::Cube;

pub struct Renderer {
    dot_radius: i32,
    bg_color: Color,
    fg_color: Color,
}

impl Renderer {
    pub fn new(dot_radius: i32) -> Self {
        Self {
            dot_radius,
            bg_color: Color::RGB(0, 0, 0),
            fg_color: Color::RGB(255, 255, 255),
        }
    }

    pub fn draw_frame(
        &self,
        canvas: &mut Canvas<Window>,
        cube: &Cube,
    ) -> Result<(), sdl3::Error> {
        canvas.set_draw_color(self.bg_color);
        canvas.clear();
        canvas.set_draw_color(self.fg_color);

        let verts = cube.projected_vertices();

        // Edges first so dots render on top
        for &(a, b) in cube.edges() {
            let (x1, y1) = verts[a];
            let (x2, y2) = verts[b];
            canvas.draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))?;
        }

        for &(vx, vy) in &verts {
            self.draw_dot(canvas, vx, vy)?;
        }

        canvas.present();
        Ok(())
    }

    fn draw_dot(
        &self,
        canvas: &mut Canvas<Window>,
        x: f32,
        y: f32,
    ) -> Result<(), sdl3::Error> {
        let r = self.dot_radius;
        for dx in -(r - 1)..r {
            for dy in -(r - 1)..r {
                if dx * dx + dy * dy <= r * r {
                    canvas.draw_point(((x + dx as f32) as i32, (y + dy as f32) as i32))?;
                }
            }
        }
        Ok(())
    }
}