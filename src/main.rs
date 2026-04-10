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
    size: i32,
) -> Result<(), sdl3::Error> {
    let (x, y) = v.to_point();

    let rect = Rect::new(
        x - size / 2,
        y - size / 2,
        size as u32,
        size as u32,
    );

    canvas.fill_rect(rect)
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

    let mut vertex = Vertex::new(400.0, 300.0);

    'running: loop {
        // -------- EVENTS --------
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => vertex = vertex.translate(0.0, -10.0),

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => vertex = vertex.translate(0.0, 10.0),

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => vertex = vertex.translate(-10.0, 0.0),

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => vertex = vertex.translate(10.0, 0.0),

                _ => {}
            }
        }

        // -------- RENDER --------
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        draw_vertex(&mut canvas, vertex, 10)?;

        canvas.present();
    }

    Ok(())
}