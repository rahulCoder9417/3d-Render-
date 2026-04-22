use crate::math::{rotation_xyz, Vertex};

pub const EDGES: [(usize, usize); 12] = [
    (0, 1), (0, 2), (1, 3), (2, 3), // front face
    (4, 5), (4, 6), (5, 7), (6, 7), // back face
    (0, 4), (1, 5), (2, 6), (3, 7), //  edges
];

pub struct Cube {
    vertices: [Vertex; 8],
    center: Vertex,
    pub rotation_angle: f32,
}

impl Cube {
    /// Build a cube from an origin corner (ox, oy, oz) and a side size.
    pub fn new(ox: f32, oy: f32, oz: f32, size: f32, depth: (f32, f32, f32)) -> Self {
        let (x_depth, y_depth, z_depth) = depth;
        let s = size;
        let vertices = [
            Vertex::new(ox              , oy              , oz              ),
            Vertex::new(ox              , oy + s + y_depth, oz              ),
            Vertex::new(ox + s + x_depth, oy              , oz              ),
            Vertex::new(ox + s + x_depth, oy + s + y_depth, oz              ),
            Vertex::new(ox              , oy              , oz + s + z_depth),
            Vertex::new(ox              , oy + s + y_depth, oz + s + z_depth),
            Vertex::new(ox + s + x_depth, oy              , oz + s + z_depth),
            Vertex::new(ox + s + x_depth, oy + s + y_depth, oz + s + z_depth),
        ];

        let center = Self::compute_center(&vertices);
        Self { vertices, center, rotation_angle: 0.0 }
    }

    pub fn tick(&mut self, delta: f32) {
        self.rotation_angle += delta;
    }

    /// Returns world-space vertices after the cube's self-rotation.
    pub fn world_vertices(&self) -> [Vertex; 8] {
        let rot = rotation_xyz(self.rotation_angle);
        let (cx, cy, cz) = (self.center.x, self.center.y, self.center.z);

        let mut out = [Vertex::new(0.0, 0.0, 0.0); 8];
        for (i, v) in self.vertices.iter().enumerate() {
            let local = v.translate(-cx, -cy, -cz);
            let rotated = local.transform(&rot);
            out[i] = rotated.translate(cx, cy, cz);
        }
        out
    }

    pub fn edges(&self) -> &[(usize, usize); 12] {
        &EDGES
    }

    fn compute_center(verts: &[Vertex; 8]) -> Vertex {
        let n = verts.len() as f32;
        Vertex::new(
            verts.iter().map(|v| v.x).sum::<f32>() / n,
            verts.iter().map(|v| v.y).sum::<f32>() / n,
            verts.iter().map(|v| v.z).sum::<f32>() / n,
        )
    }
}
