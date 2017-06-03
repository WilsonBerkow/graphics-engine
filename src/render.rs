use matrix::Matrix;
use std::fmt;
use consts::*;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }

    pub fn white() -> Color {
        Color::rgb(255, 255, 255)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn xy(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }

    #[allow(dead_code)]
    pub fn vector_sum(&self, p: Point) -> Point {
        Point { x: self.x + p.x, y: self.y + p.y }
    }

    pub fn vector_diff(&self, p: Point) -> Point {
        Point { x: self.x - p.x, y: self.y - p.y }
    }

    pub fn clockwise_of(&self, p: Point) -> bool {
        p.x * self.y - p.y * self.x > 0
    }
}

/// Draw edges in an edge list matrix. Each successive pair of
/// columns are considered the endpoints of a distinct edge
/// (i.e. [A-start | A-end | B-start | B-end | etc...]).
///
/// All edges are drawn in white.
pub fn edge_list(image: &mut Vec<Vec<Color>>, edges: &Matrix) {
    let mut c = 0;
    while c + 1 < edges.width() {
        let pcol = edges.col(c);
        let qcol = edges.col(c + 1);
        let p = Point::xy(pcol[0] as i64, pcol[1] as i64);
        let q = Point::xy(qcol[0] as i64, qcol[1] as i64);
        line(image, p, q, Color::white());
        c += 2;
    }
}

pub fn triangle_list(image: &mut Vec<Vec<Color>>, edges: &Matrix) {
    let mut i = 0;
    while i + 2 < edges.width() {
        let pcol = edges.col(i);
        let p = Point::xy(pcol[0] as i64, pcol[1] as i64);
        let qcol = edges.col(i + 1);
        let q = Point::xy(qcol[0] as i64, qcol[1] as i64);
        let rcol = edges.col(i + 2);
        let r = Point::xy(rcol[0] as i64, rcol[1] as i64);
        if r.vector_diff(p).clockwise_of(q.vector_diff(p)) {
            line(image, p, q, Color::white());
            line(image, q, r, Color::white());
            line(image, r, p, Color::white());
        }
        i += 3;
    }
}

/// Draw a line in `image` using Bresenham's line algorithm (and variants for each octant).
pub fn line(image: &mut Vec<Vec<Color>>, start: Point, end: Point, color: Color) {
    if start.x > end.x {
        // Swap `start` and `end` so `start` is on the left
        line(image, end, start, color);
    } else {
        // Dispatch to various functions based on octant
        let more_vertical = (end.y - start.y).abs() > (end.x - start.x).abs();
        if end.y > start.y {
            if more_vertical {
                bline_oct2(image, start, end, color);
            } else {
                bline_oct1(image, start, end, color);
            }
        } else {
            if more_vertical {
                bline_oct7(image, start, end, color);
            } else {
                bline_oct8(image, start, end, color);
            }
        }
    }
}

fn within_screen(image: &mut Vec<Vec<Color>>, p: Point) -> bool {
    let within_y = p.y >= 0 && p.y < image.len() as i64;
    let within_x = p.x >= 0 && image.len() > 0 && p.x < image[0].len() as i64;
    within_y && within_x
}

/// If the point `p` is within the width and height of `image`, plot `color` at `p`.
pub fn plot_if_visible(image: &mut Vec<Vec<Color>>, p: Point, color: Color) {
    if within_screen(image, p) {
        image[HEIGHT - (p.y + 1) as usize][p.x as usize] = color;
    }
}

/// Bresenham's Line Algorithm for octant 1
fn bline_oct1(image: &mut Vec<Vec<Color>>, mut start: Point, end: Point, color: Color) {
    let dx: i64 = end.x - start.x;
    let dy: i64 = end.y - start.y;
    let mut d: i64 = 2 * dy - dx;
    // move `start` along the line and plot it as we go
    while start.x <= end.x {
        plot_if_visible(image, start, color);
        start.x += 1;
        d += dy;
        if d > 0 {
            start.y += 1;
            d -= dx;
        }
    }
}

/// Bresenham's Line Algorithm for octant 2
fn bline_oct2(image: &mut Vec<Vec<Color>>, mut start: Point, end: Point, color: Color) {
    let dx: i64 = end.x - start.x;
    let dy: i64 = end.y - start.y;
    let mut d: i64 = 2 * dy - dx;
    // move `start` along the line and plot it as we go
    while start.y <= end.y {
        plot_if_visible(image, start, color);
        if d > 0 {
            start.x += 1;
            d -= dy;
        }
        start.y += 1;
        d += dx;
    }
}

/// Bresenham's Line Algorithm for octant 7
fn bline_oct7(image: &mut Vec<Vec<Color>>, mut start: Point, end: Point, color: Color) {
    let dx: i64 = end.x - start.x;
    let dy: i64 = end.y - start.y;
    let mut d: i64 = dy + 2 * dx;
    let b = -2 * dx;
    let a = 2 * dy;
    // move `start` along the line and plot it as we go
    while start.y >= end.y {
        plot_if_visible(image, start, color);
        if d > 0 {
            start.x += 1;
            d += a;
        }
        start.y -= 1;
        d -= b;
    }
}

/// Bresenham's Line Algorithm for octant 8
fn bline_oct8(image: &mut Vec<Vec<Color>>, mut start: Point, end: Point, color: Color) {
    let dx: i64 = end.x - start.x;
    let dy: i64 = end.y - start.y;
    let mut d: i64 = 2 * dy + dx;
    let a = 2 * dy;
    let b = -2 * dx;
    // move `start` along the line and plot it as we go
    while start.x <= end.x {
        plot_if_visible(image, start, color);
        if d < 0 {
            start.y -= 1;
            d -= b;
        }
        start.x += 1;
        d += a;
    }
}
