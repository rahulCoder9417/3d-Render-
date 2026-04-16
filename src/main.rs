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
    let oy = height as f32 / 3.5;
    let mut vec_cube = vec![Cube::new(ox, oy, 0.0, size, (24.0, 118.0, 45.0)),
                             Cube::new(ox, oy, 50.0, size, (0.0, 0.0, 0.0)),  ];
    // Place sphere to the left of the cube
    let sphere_center = Vertex::new(ox+10.0 , oy+228.0, 0.0);
    let mut vec_sphere = vec![Sphere::new(sphere_center, size / 2.0, 6, 10)];

    let renderer = Renderer::new(4);
let mut last_time = Instant::now();

'running: loop {
    let now = Instant::now();
    let dt = now.duration_since(last_time).as_secs_f32();
    last_time = now;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
            _ => {}
        }
    }
    let speed = 1.5;
    for cube in &mut vec_cube {
        cube.tick(speed * dt);
    }
    for sphere in &mut vec_sphere {
        sphere.tick(speed * dt);
    }

    renderer.draw_frame(&mut canvas, &vec_cube, &vec_sphere)?;
}

    Ok(())
}