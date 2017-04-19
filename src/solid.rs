use matrix::Matrix;
use std::f64::consts::PI;

fn push_vertex(edges: &mut Matrix, x: f64, y: f64, z: f64) {
    edges.push_edge([x, y, z, 1.0], [x, y, z, 1.0]);
}

// The points should be passed in clockwise order, and they will be
// added to the list clockwise
fn push_quad(edges: &mut Matrix, tl: [f64; 4], tr: [f64; 4], br: [f64; 4], bl: [f64; 4]) {
    edges.push_triangle(
        [tl[0], tl[1], tl[2], tl[3]],
        [tr[0], tr[1], tr[2], tr[3]],
        [br[0], br[1], br[2], br[3]]);
    edges.push_triangle(
        [tl[0], tl[1], tl[2], tl[3]],
        [br[0], br[1], br[2], tr[3]],
        [bl[0], bl[1], bl[2], br[3]]);
}

/// Generate the triangles of a rectangular prism whose front-upper-left vertex
/// is (x, y, z) and whose dimensions are (dx, dy, dz).
pub fn rect_prism(triangles: &mut Matrix, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) {
    // Front face:
    push_quad(triangles,
        [x, y, z, 1.0],
        [x + dx, y, z, 1.0],
        [x + dx, y + dy, z, 1.0],
        [x, y + dy, z, 1.0]);
    // Back face:
    push_quad(triangles,
        [x, y, z + dz, 1.0],
        [x, y + dy, z + dz, 1.0],
        [x + dx, y + dy, z + dz, 1.0],
        [x + dx, y, z + dz, 1.0]);
    // Left face:
    push_quad(triangles,
        [x, y, z, 1.0],
        [x, y + dy, z, 1.0],
        [x, y + dy, z + dz, 1.0],
        [x, y, z + dz, 1.0]);
    // Right face:
    push_quad(triangles,
        [x + dx, y, z, 1.0],
        [x + dx, y, z + dz, 1.0],
        [x + dx, y + dy, z + dz, 1.0],
        [x + dx, y + dy, z, 1.0]);
    // Top face:
    push_quad(triangles,
        [x, y, z, 1.0],
        [x, y, z + dz, 1.0],
        [x + dx, y, z + dz, 1.0],
        [x + dx, y, z, 1.0]);
    // Bottom face:
    push_quad(triangles,
        [x, y + dy, z, 1.0],
        [x + dx, y + dy, z, 1.0],
        [x + dx, y + dy, z + dz, 1.0],
        [x, y + dy, z + dz, 1.0]);
}

pub fn sphere(triangles: &mut Matrix, cx: f64, cy: f64, cz: f64, r: f64) {
    let mut sphere_points = vec![];
    let semicircles = 16;
    let points_per_semi = 10; // points per semicircle
    // Generate `semicircles` semicircles (comprising the sphere)
    for semicirc in 0..semicircles {
        // `a` is the angle of rotation of this semicircle
        let a = semicirc as f64 / semicircles as f64 * 2.0 * PI;
        let cos_a = a.cos();
        let sin_a = a.sin();
        // Generate the points on this semicircle
        for pt in 0..points_per_semi + 1 {
            let b = pt as f64 / points_per_semi as f64 * PI;
            let cos_b = b.cos();
            let sin_b = b.sin();
            sphere_points.push([
                cx + r * cos_b,
                cy + r * sin_b * cos_a,
                cz + r * sin_b * sin_a,
                1.0]);
        }
    }
    // Add the triangles of the sphere to the triangle matrix
    let len = sphere_points.len();
    for i in 0..len {
        // TODO: Skip degenerate triangles occuring at the poles (they're benign for now)
        // Draw a quadrilateral on this part of the sphere
        push_quad(triangles,
                  sphere_points[i],
                  sphere_points[(i + 1) % len],
                  sphere_points[(i + points_per_semi + 1) % len],
                  sphere_points[(i + points_per_semi) % len]);
    }
}

pub fn torus(triangles: &mut Matrix, x: f64, y: f64, z: f64, r1: f64, r2: f64) {
    let mut torus_points = vec![];
    let circles = 20;
    let pts_per_circ = 16;
    // Populate `torus_points` with points of the torus
    for circ in 0..circles {
        let phi = circ as f64 / circles as f64 * 2.0 * PI;
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        for pt in 0..pts_per_circ {
            let theta = pt as f64 / pts_per_circ as f64 * 2.0 * PI;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            torus_points.push([
                x + cos_phi * (r1 * cos_theta + r2),
                y + r2 * sin_theta,
                z + sin_phi * (r1 * cos_theta + r2),
                1.0]);
        }
    }
    // Add torus to triangle list
    let len = torus_points.len();
    for i in 0..len {
        push_quad(triangles,
            torus_points[i],
            torus_points[(i + pts_per_circ) % len],
            torus_points[(i + pts_per_circ + 1) % len],
            torus_points[(i + 1) % len]);
    }
}
