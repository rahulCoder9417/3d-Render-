use sdl3::event::Event;
use sdl3::keyboard::Keycode;

use render_from_scratch::{Cube, Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video = sdl_context.video()?;
    let (width, height) = (800u32, 600u32);

    let window = video
        .window("Renderer", width, height)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    // Build the cube
    let size = width as f32 / 7.0;
    let ox = width as f32 / 2.5;
    let oy = height as f32 / 2.5;
    let mut cube = Cube::new(ox, oy, 0.0, size);

    let renderer = Renderer::new(4);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        cube.tick(0.0005);
        renderer.draw_frame(&mut canvas, &cube)?;
    }

    Ok(())
}