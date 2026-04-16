use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Instant;
use render_from_scratch::{Cube, Sphere, Renderer, Vertex};

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

    let size = width as f32 / 7.0;
    let ox = width as f32 / 2.5;
    let oy = height as f32 / 2.5;
    let mut cube = Cube::new(ox, oy, 0.0, size);

    // Place sphere to the left of the cube
    let sphere_center = Vertex::new(ox - size * 2.0, oy + size / 2.0, 0.0);
    let mut sphere = Sphere::new(sphere_center, size / 2.0, 6, 10);

    let renderer = Renderer::new(4);
let mut last_time = Instant::now();

'running: loop {
    let now = Instant::now();
    let dt = now.duration_since(last_time).as_secs_f32();
    last_time = now;

    let speed = 1.5;

    cube.tick(speed * dt);
    sphere.tick(speed * dt);

    renderer.draw_frame(&mut canvas, &cube, &sphere)?;
}

    Ok(())
}