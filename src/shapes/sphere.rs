use crate::math::{rotation_xyz, Vertex};

pub struct Sphere {
    center: Vertex,
    pub rotation_angle: f32,
    vertices: Vec<Vertex>,
    edges: Vec<(usize, usize)>,
}

impl Sphere {
    // stacks = horizontal rings, slices = vertical segments
    pub fn new(center: Vertex, radius: f32, stacks: usize, slices: usize) -> Self {
        let mut vertices = Vec::with_capacity((stacks + 1) * slices);
        let mut edges = Vec::new();

        for stack in 0..=stacks {
            let phi = std::f32::consts::PI * stack as f32 / stacks as f32;
            for slice in 0..slices {
                let theta = 2.0 * std::f32::consts::PI * slice as f32 / slices as f32;
                let x = center.x + radius * phi.sin() * theta.cos();
                let y = center.y + radius * phi.sin() * theta.sin();
                let z = center.z + radius * phi.cos();
                vertices.push(Vertex::new(x, y, z));
            }
        }

        for stack in 0..=stacks {
            for slice in 0..slices {
                let current = stack * slices + slice;
                let next = stack * slices + (slice + 1) % slices;
                edges.push((current, next));
            }
        }

        for stack in 0..stacks {
            for slice in 0..slices {
                let current = stack * slices + slice;
                let below = (stack + 1) * slices + slice;
                edges.push((current, below));
            }
        }

        Self { center, rotation_angle: 0.0, vertices, edges }
    }

    pub fn tick(&mut self, delta: f32) {
        self.rotation_angle += delta;
    }

    pub fn world_vertices(&self) -> Vec<Vertex> {
        let rot = rotation_xyz(self.rotation_angle);
        let (cx, cy, cz) = (self.center.x, self.center.y, self.center.z);

        self.vertices.iter().map(|v| {
            let local = v.translate(-cx, -cy, -cz);
            let rotated = local.transform(&rot);
            rotated.translate(cx, cy, cz)
        }).collect()
    }

    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
}
