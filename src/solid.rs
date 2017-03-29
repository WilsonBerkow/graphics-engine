use matrix::Matrix;
use std::f64::consts::PI;

fn push_vertex(edges: &mut Matrix, x: f64, y: f64, z: f64) {
    edges.push_edge([x, y, z, 1.0], [x, y, z, 1.0]);
}

/// Draw a rectangular prism whose front-upper-left vertex is (x, y, z) and
/// whose dimensions are (dx, dy, dz).
pub fn rect_prism(edges: &mut Matrix, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) {
    // Front square:
    push_vertex(edges, x, y, z);
    push_vertex(edges, x + dx, y, z);
    push_vertex(edges, x, y + dy, z);
    push_vertex(edges, x + dx, y + dy, z);
    // Back square:
    push_vertex(edges, x, y, z + dz);
    push_vertex(edges, x + dx, y, z + dz);
    push_vertex(edges, x, y + dy, z + dz);
    push_vertex(edges, x + dx, y + dy, z + dz);
}

pub fn sphere(edges: &mut Matrix, cx: f64, cy: f64, cz: f64, r: f64) {
    let semicircles = 24;
    let points = 80; // points per semicircle
    for semicirc in 0..semicircles {
        // `a` is the angle of rotation of this semicircle
        let a = semicirc as f64 / semicircles as f64 * 2.0 * PI;
        let cos_a = a.cos();
        let sin_a = a.sin();
        for pt in 0..points {
            let b = pt as f64 / points as f64 * PI;
            let cos_b = b.cos();
            let sin_b = b.sin();
            push_vertex(
                edges,
                cx + r * cos_b,
                cy + r * sin_b * cos_a,
                cz + r * sin_b * sin_a);
        }
    }
}

pub fn torus(edges: &mut Matrix, x: f64, y: f64, z: f64, r1: f64, r2: f64) {
    let circles = 100;
    let points = 80; // per circle
    for circ in 0..circles {
        let phi = circ as f64 / circles as f64 * 2.0 * PI;
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        for pt in 0..points {
            let theta = pt as f64 / points as f64 * 2.0 * PI;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            push_vertex(
                edges,
                x + cos_phi * (r1 * cos_theta + r2),
                y + r2 * sin_theta,
                z + sin_phi * (r1 * cos_theta + r2));
        }
    }
}
