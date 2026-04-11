use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

use render_from_scratch::Vertex;

enum Axis {
    X,
    Y,
    Z,
}

fn draw_vertex(canvas: &mut Canvas<Window>, x: f32, y: f32, radius: i32) -> Result<(), sdl3::Error> {
    for dx in -(radius - 1)..radius {
        for dy in -(radius - 1)..radius {
            if dx * dx + dy * dy <= radius * radius {
                canvas.draw_point(((x + dx as f32) as i32, (y + dy as f32) as i32))?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video = sdl_context.video()?;
    let (height, width) = (600u32, 800u32);
    let window = video
        .window("Renderer", width, height)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    let radius = 4;
    let mut rotation_angle: f32 = 0.0;
    //  projection matrix (3x3, drops Z)
    let projection_matrix = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];

    let (x, y, offset) = (
        width as f32 / 2.5,
        height as f32 / 2.5,
        width as f32 / 7.0,
    );

    let vertices: Vec<Vertex> = vec![
        Vertex::new(x,          y,          0.0),
        Vertex::new(x,          y + offset, 0.0),
        Vertex::new(x + offset, y,          0.0),
        Vertex::new(x + offset, y + offset, 0.0),
    ];

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
 // Combine rotation matrices: Rz * Ry * Rx
            let rotation_matrix = multiply_matrix(
                &multiply_matrix(
                    &get_rotation_matrix(rotation_angle, Axis::Z),
                    &get_rotation_matrix(rotation_angle, Axis::Y),
                ),
                &get_rotation_matrix(rotation_angle, Axis::X),
            );
    // Compute center of the shape
let cx = vertices.iter().map(|v| v.x).sum::<f32>() / vertices.len() as f32;
let cy = vertices.iter().map(|v| v.y).sum::<f32>() / vertices.len() as f32;
let cz = vertices.iter().map(|v| v.z).sum::<f32>() / vertices.len() as f32;

for vertex in &vertices {
    let vertex_col: Vec<Vec<f32>> = vec![
        vec![vertex.x - cx],
        vec![vertex.y - cy],
        vec![vertex.z - cz],
    ];

    let rotated = multiply_matrix(&rotation_matrix, &vertex_col);

    let projected = multiply_matrix(&projection_matrix, &rotated);

    draw_vertex(
        &mut canvas,
        projected[0][0] + cx,
        projected[1][0] + cy,
        radius,
    )?;
}

        canvas.present();
        rotation_angle += 0.0002;
    }

    Ok(())
}

fn multiply_matrix(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();
    assert_eq!(cols_a, b.len(), "Matrix dimensions do not match");

    let mut result = vec![vec![0.0_f32; cols_b]; rows_a];
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn get_rotation_matrix(angle: f32, axis: Axis) -> Vec<Vec<f32>> {
    let cos = angle.cos();
    let sin = angle.sin();

    match axis {
        Axis::X => vec![
            vec![1.0, 0.0,  0.0],
            vec![0.0, cos, -sin],
            vec![0.0, sin,  cos],
        ],
        Axis::Y => vec![
            vec![ cos, 0.0, sin],
            vec![ 0.0, 1.0, 0.0],
            vec![-sin, 0.0, cos],
        ],
        Axis::Z => vec![
            vec![cos, -sin, 0.0],
            vec![sin,  cos, 0.0],
            vec![0.0,  0.0, 1.0],
        ],
    }
}