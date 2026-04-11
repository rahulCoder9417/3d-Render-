use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use render_from_scratch::Vertex;

fn draw_vertex(
    canvas: &mut Canvas<Window>,
    v: Vertex,
    radius: i32,
) -> Result<(), sdl3::Error> {
    let (x, y) = v.to_point();
    for dx in -(radius-1)..radius {
        for dy in -(radius-1)..radius {
            if dx * dx + dy * dy <= radius * radius {
                canvas.draw_point((x + dx, y + dy))?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window("Renderer", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    let mut vertex = Vertex::new(400.0, 300.0,0.0);

    'running: loop {
        // -------- EVENTS --------
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        // -------- RENDER --------
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        draw_vertex(&mut canvas, vertex, 5)?;

        canvas.present();
    }

    Ok(())
}