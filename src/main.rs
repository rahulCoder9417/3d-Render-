use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use render_from_scratch::Vertex;
enum Axis {
    X,
    Y,
    Z,
}
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
    let (HEIGHT, WIDTH) = (600, 800);
    let window = video
        .window("Renderer", WIDTH, HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    //projection matrix
    let projection_matrix = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0]
    ];

    let (x,y,offset) = (WIDTH as f32 / 2.5, HEIGHT as f32 / 2.5, WIDTH as f32 / 7.0);
    let mut Vertexs: Vec<Vertex> = vec![];
    Vertexs.push(Vertex::new(x, y, 0.0));
    Vertexs.push(Vertex::new(x, y + offset, 0.0));
    Vertexs.push(Vertex::new(x + offset, y, 0.0));
    Vertexs.push(Vertex::new(x + offset, y + offset, 0.0));

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

        for vertex in &Vertexs {
            draw_vertex(&mut canvas, *vertex, 4)?;
        }

        canvas.present();
    }

    Ok(())
}

fn multipy_matrix(a:&Vec<Vec<f32>>, b:&Vec<Vec<f32>>) -> Vec<Vec<f32>> {
   let rows_a = a.len();
   let cols_a = a[0].len();
   let rows_b = b.len();
   let cols_b = b[0].len();
   if(cols_a != rows_b) {
       panic!("Matrix dimensions do not match");
   }
   let mut result = vec![vec![0.0; cols_b]; rows_a];
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
            vec![1.0, 0.0, 0.0],
            vec![0.0, cos, -sin],
            vec![0.0, sin, cos],
        ],

        Axis::Y => vec![
            vec![cos, 0.0, sin],
            vec![0.0, 1.0, 0.0],
            vec![-sin, 0.0, cos],
        ],

        Axis::Z => vec![
            vec![cos, -sin, 0.0],
            vec![sin, cos, 0.0],
            vec![0.0, 0.0, 1.0],
        ],
    }
}