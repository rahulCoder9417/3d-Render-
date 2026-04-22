use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::camera::Camera;
use crate::math::Vertex;
use crate::shapes::{Cube, Sphere};

pub struct Renderer {
    dot_radius: i32,
    bg_color: Color,
    cube_color: Color,
    sphere_color: Color,
}

impl Renderer {
    pub fn new(dot_radius: i32) -> Self {
        Self {
            dot_radius,
            bg_color: Color::RGB(0, 0, 0),
            cube_color: Color::RGB(255, 255, 255),
            sphere_color: Color::RGB(100, 200, 255),
        }
    }

    pub fn draw_frame(
        &self,
        canvas: &mut Canvas<Window>,
        camera: &Camera,
        screen: (f32, f32),
        cubes: &[Cube],
        spheres: &[Sphere],
    ) -> Result<(), sdl3::Error> {
        canvas.set_draw_color(self.bg_color);
        canvas.clear();

        canvas.set_draw_color(self.cube_color);
        for cube in cubes {
            let verts = self.project_all(camera, screen, &cube.world_vertices());
            self.draw_edges(canvas, &verts, cube.edges())?;
            self.draw_dots(canvas, &verts)?;
        }

        canvas.set_draw_color(self.sphere_color);
        for sphere in spheres {
            let verts = self.project_all(camera, screen, &sphere.world_vertices());
            self.draw_edges(canvas, &verts, sphere.edges())?;
        }

        canvas.present();
        Ok(())
    }

    fn project_all(
        &self,
        camera: &Camera,
        screen: (f32, f32),
        verts: &[Vertex],
    ) -> Vec<Option<(f32, f32)>> {
        verts.iter().map(|v| camera.project(*v, screen.0, screen.1)).collect()
    }

    fn draw_edges(
        &self,
        canvas: &mut Canvas<Window>,
        verts: &[Option<(f32, f32)>],
        edges: &[(usize, usize)],
    ) -> Result<(), sdl3::Error> {
        for &(a, b) in edges {
            if let (Some((x1, y1)), Some((x2, y2))) = (verts[a], verts[b]) {
                canvas.draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))?;
            }
        }
        Ok(())
    }

    fn draw_dots(
        &self,
        canvas: &mut Canvas<Window>,
        verts: &[Option<(f32, f32)>],
    ) -> Result<(), sdl3::Error> {
        for v in verts.iter().flatten() {
            self.draw_dot(canvas, v.0, v.1)?;
        }
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
