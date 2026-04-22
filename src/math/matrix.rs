pub type Mat3 = [[f32; 3]; 3];
pub type Col3 = [f32; 3];

pub fn mat3_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut out = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    out
}

pub fn mat3_mul_col(m: &Mat3, v: &Col3) -> Col3 {
    [
        m[0][0]*v[0] + m[0][1]*v[1] + m[0][2]*v[2],
        m[1][0]*v[0] + m[1][1]*v[1] + m[1][2]*v[2],
        m[2][0]*v[0] + m[2][1]*v[1] + m[2][2]*v[2],
    ]
}


pub fn rotation_x(a: f32) -> Mat3 {
    let (s, c) = (a.sin(), a.cos());
    [[1.0, 0.0, 0.0],
     [0.0,   c,  -s],
     [0.0,   s,   c]]
}

pub fn rotation_y(a: f32) -> Mat3 {
    let (s, c) = (a.sin(), a.cos());
    [[ c,  0.0,  s],
     [0.0, 1.0, 0.0],
     [-s,  0.0,  c]]
}

pub fn rotation_z(a: f32) -> Mat3 {
    let (s, c) = (a.sin(), a.cos());
    [[c,  -s,  0.0],
     [s,   c,  0.0],
     [0.0, 0.0, 1.0]]
}

/// Combined Rz * Ry * Rx
pub fn rotation_xyz(angle: f32) -> Mat3 {
    mat3_mul(
        &mat3_mul(&rotation_z(angle), &rotation_y(angle)),
        &rotation_x(angle),
    )
}