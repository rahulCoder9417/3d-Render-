use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Point;

use render_from_scratch::Vertex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window("Vertex Renderer", 800, 600)
        .position_centered()
        .build()?; 

    let mut canvas = window.into_canvas();

    let mut event_pump = sdl_context.event_pump()?;

    let vertex = Vertex::new(400.0, 300.0);

    'running: loop {
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_point(Point::new(vertex.x as i32, vertex.y as i32))?;

        canvas.present();
    }

    Ok(())
}