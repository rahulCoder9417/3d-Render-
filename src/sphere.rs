use crate::math::{rotation_xyz, PROJECTION};
use crate::vertex::Vertex;

pub struct Sphere {
    center: Vertex,
    radius: f32,
    pub rotation_angle: f32,
    vertices: Vec<Vertex>,
    pub edges: Vec<(usize, usize)>,
}

impl Sphere {
    //stack-horizontal rings, slices = vertical segments
    pub fn new(center: Vertex, radius: f32, stacks: usize, slices: usize) -> Self {
        let mut vertices = Vec::new();
        let mut edges = Vec::new();

        // Generate vertices using spherical coordinates
        for stack in 0..=stacks {
            let phi = std::f32::consts::PI * stack as f32 / stacks as f32; // 0 to PI
            for slice in 0..slices {
                let theta = 2.0 * std::f32::consts::PI * slice as f32 / slices as f32; // 0 to 2PI
                let x = center.x + radius * phi.sin() * theta.cos();
                let y = center.y + radius * phi.sin() * theta.sin();
                let z = center.z + radius * phi.cos();
                vertices.push(Vertex::new(x, y, z));
            }
        }

        // Horizontal ring edges
        for stack in 0..=stacks {
            for slice in 0..slices {
                let current = stack * slices + slice;
                let next = stack * slices + (slice + 1) % slices;
                edges.push((current, next));
            }
        }

        // Vertical column edges
        for stack in 0..stacks {
            for slice in 0..slices {
                let current = stack * slices + slice;
                let below = (stack + 1) * slices + slice;
                edges.push((current, below));
            }
        }

        Self { center, radius, rotation_angle: 0.0, vertices, edges }
    }

    pub fn tick(&mut self, delta: f32) {
        self.rotation_angle += delta;
    }

    pub fn projected_vertices(&self) -> Vec<(f32, f32)> {
        let rot = rotation_xyz(self.rotation_angle);
        let cx = self.center.x;
        let cy = self.center.y;
        let cz = self.center.z;

        self.vertices.iter().map(|v| {
            let local = v.translate(-cx, -cy, -cz);
            let rotated = local.transform(&rot);
            let projected = rotated.transform(&PROJECTION);
            (projected.x + cx, projected.y + cy)
        }).collect()
    }
}