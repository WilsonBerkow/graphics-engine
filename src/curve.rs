#![allow(dead_code)]
// For the moment curves are not being used

use std::f64::consts::PI;

use matrix::Matrix;

/// Add a parametric curve with `points` points to `edges`.
pub fn parametric<F, G, H>(edges: &mut Matrix, points: usize, x: F, y: G, z: H)
    where F: Fn(f64) -> f64,
          G: Fn(f64) -> f64,
          H: Fn(f64) -> f64
{
    // Set the initial point
    let mut prev_point = [x(0.0), y(0.0), z(0.0), 1.0];

    // Iterate over each point, connecting the dots
    for i in 1..(points + 1) {
        let t = i as f64 / points as f64;
        let new_point = [x(t), y(t), z(t), 1.0];
        edges.push_edge(prev_point, new_point);
        prev_point = new_point;
    }
}

pub fn circle(edges: &mut Matrix, cx: f64, cy: f64, cz: f64, r: f64) {
    let points = 6 * r as usize; // Almost 2 * pi * r
    parametric(
        edges,
        points,
        |t| cx + r * (2.0 * PI * t).cos(), // x function
        |t| cy + r * (2.0 * PI * t).sin(), // y function
        |_t| cz); // z function
}

fn bezier_matrix() -> Matrix {
    Matrix::new4x4(
        -1.0,  3.0, -3.0, 1.0,
         3.0, -6.0,  3.0, 0.0,
        -3.0,  3.0,  0.0, 0.0,
         1.0,  0.0,  0.0, 0.0)
}

/// Add a bezier curve to `edges`, approximated with `points` points.
pub fn bezier(edges: &mut Matrix, points: usize, p0: [f64; 4], p1: [f64; 4], p2: [f64; 4], p3: [f64; 4]) {
    let bezier_mat = bezier_matrix();

    let x_coefficients = &bezier_mat * &Matrix::column_vector(p0[0], p1[0], p2[0], p3[0]);
    let y_coefficients = &bezier_mat * &Matrix::column_vector(p0[1], p1[1], p2[1], p3[1]);
    let z_coefficients = &bezier_mat * &Matrix::column_vector(p0[2], p1[2], p2[2], p3[2]);

    let x_fn = |t| apply_cubic_coefficients(&x_coefficients, t);
    let y_fn = |t| apply_cubic_coefficients(&y_coefficients, t);
    let z_fn = |t| apply_cubic_coefficients(&z_coefficients, t);
    parametric(edges, points, x_fn, y_fn, z_fn);
}

// the hermite_matrix (H-inverse) times the [p0, p1, m0, m1] matrix (G) creates the coefficient
// matrix [a, b, c, d]
fn hermite_matrix() -> Matrix {
    Matrix::new4x4(
        2.0, -2.0,  1.0,  1.0,
       -3.0,  3.0, -2.0, -1.0,
        0.0,  0.0,  1.0,  0.0,
        1.0,  0.0,  0.0,  0.0)
}

/// Add a hermite curve to `edges`, approximated with `points` points.
pub fn hermite(edges: &mut Matrix, points: usize, p0: [f64; 4], p1: [f64; 4], m0: [f64; 4], m1: [f64; 4]) {
    let hermite_mat = hermite_matrix();

    let x_coefficients = &hermite_mat * &Matrix::column_vector(p0[0], p1[0], m0[0], m1[0]);
    let y_coefficients = &hermite_mat * &Matrix::column_vector(p0[1], p1[1], m0[1], m1[1]);
    let z_coefficients = &hermite_mat * &Matrix::column_vector(p0[2], p1[2], m0[2], m1[2]);

    let x_fn = |t| apply_cubic_coefficients(&x_coefficients, t);
    let y_fn = |t| apply_cubic_coefficients(&y_coefficients, t);
    let z_fn = |t| apply_cubic_coefficients(&z_coefficients, t);
    parametric(edges, points, x_fn, y_fn, z_fn);
}

fn apply_cubic_coefficients(coefficients: &Matrix, t: f64) -> f64 {
    let a = coefficients.get(0, 0);
    let b = coefficients.get(1, 0);
    let c = coefficients.get(2, 0);
    let d = coefficients.get(3, 0);
    a * t.powi(3) + b * t.powi(2) + c * t + d
}
