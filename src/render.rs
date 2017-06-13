use std::mem::swap;
use matrix::Matrix;
use std::fmt;
use consts::*;

// row-major order
// The size of the [u8] is WIDTH * HEIGH T* PX_SIZE. A slice is used because
// allocating an array directly on the heap seems to require excessive jankiness.
pub struct Screen(Box<[u8]>);

const SCREEN_ROW_SIZE: usize = WIDTH * PX_SIZE;

impl Screen {
    pub fn new() -> Screen {
        // Use a Vec to allocate on the heap because Rust's heap api is
        // unstable (grumble grumble...).
        // TODO: either use an array (not slice) or allow varying screen sizes
        let vec_data = vec![0u8; WIDTH * HEIGHT * PX_SIZE];
        let data: Box<[u8]> = vec_data.into_boxed_slice();
        Screen(data)
    }

    #[allow(dead_code)]
    pub fn getxy(&self, x: usize, y: usize) -> Color {
        let row = HEIGHT - y - 1;
        let i = row * SCREEN_ROW_SIZE + x * PX_SIZE;
        Color {
            r: self.0[i],
            g: self.0[i + 1],
            b: self.0[i + 2]
        }
    }

    pub fn setxy(&mut self, x: usize, y: usize, clr: Color) {
        let row = HEIGHT - y - 1;
        let i = row * SCREEN_ROW_SIZE + x * PX_SIZE;
        self.0[i] = clr.r;
        self.0[i + 1] = clr.g;
        self.0[i + 2] = clr.b;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn clear_black(&mut self) {
        for i in 0..WIDTH * HEIGHT * PX_SIZE {
            self.0[i] = 0;
        }
    }
}

// length of [f64] is WIDTH * HEIGHT
pub struct ZBuffer(Box<[f64]>);

impl ZBuffer {
    pub fn new() -> ZBuffer {
        use std::f64::INFINITY;
        let vec_data = vec![-INFINITY; WIDTH * HEIGHT];
        ZBuffer(vec_data.into_boxed_slice())
    }

    pub fn plot(&mut self, x: usize, y: usize, z: f64) -> bool {
        let row = HEIGHT - y - 1;
        let i = row * WIDTH + x;
        if self.0[i] < z {
            self.0[i] = z;
            true
        } else {
            false
        }
    }
}

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

    #[allow(dead_code)]
    pub fn grayscale(v: u8) -> Color {
        Color::rgb(v, v, v)
    }

    #[allow(dead_code)]
    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }

    pub fn white() -> Color {
        Color::rgb(255, 255, 255)
    }

    pub fn arbitrary(i: usize) -> Color {
        // Rust's `rand` is an external crate. As my initial instructions for
        // installation of rust on Mr. DW's machine excluded Cargo, I'm not
        // using external libraries. This serves the purpose well enough.
        Color::rgb(
            (i * i * (100 - i)) as u8,
            (i * i * i) as u8,
            ((200 - i) * (150 - i)) as u8
        )
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
pub fn edge_list(image: &mut Screen, edges: &Matrix) {
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

/*pub fn triangle_list(image: &mut Screen, triangles: &Matrix) {
    let mut i = 0;
    while i + 2 < triangles.width() {
        let pcol = triangles.col(i);
        let p = Point::xy(pcol[0] as i64, pcol[1] as i64);
        let qcol = triangles.col(i + 1);
        let q = Point::xy(qcol[0] as i64, qcol[1] as i64);
        let rcol = triangles.col(i + 2);
        let r = Point::xy(rcol[0] as i64, rcol[1] as i64);
        if r.vector_diff(p).clockwise_of(q.vector_diff(p)) {
            line(image, p, q, Color::white());
            line(image, q, r, Color::white());
            line(image, r, p, Color::white());
        }
        i += 3;
    }
}*/

pub fn triangle_list(image: &mut Screen, triangles: &Matrix) {
    let mut i = 0;
    while i + 2 < triangles.width() {
        let pcol = triangles.col(i);
        let p = Point::xy(pcol[0] as i64, pcol[1] as i64);
        let qcol = triangles.col(i + 1);
        let q = Point::xy(qcol[0] as i64, qcol[1] as i64);
        let rcol = triangles.col(i + 2);
        let r = Point::xy(rcol[0] as i64, rcol[1] as i64);
        if r.vector_diff(p).clockwise_of(q.vector_diff(p)) {
            scanline(image, pcol, qcol, rcol, Color::arbitrary(i));
        }
        i += 3;
    }
}

/// Note: top, mid, and low are not required to be passed in any order.
pub fn scanline(img: &mut Screen, mut top: [f64; 4], mut mid: [f64; 4], mut low: [f64; 4], clr: Color) {
    // Sort `top`, `mid`, and `low` into the order their names imply
    if top[1] < mid[1] { swap(&mut top, &mut mid); }
    if top[1] < low[1] { swap(&mut top, &mut low); }
    if mid[1] < low[1] { swap(&mut mid, &mut low); }

    // x0 is the x pos of the edge connecting `low` to `top`
    let mut x0 = low[0];
    let dx0 = if top[0] == low[0] {
        0.0
    } else {
        (top[0] - low[0]) / (top[1] - low[1])
    };

    // x1 is the x pos of the edge connecting `low` to `mid`
    let mut x1 = low[0];
    let dx1 = if mid[0] == low[0] {
        0.0
    } else {
        (mid[0] - low[0]) / (mid[1] - low[1])
    };

    let mut z0 = low[2];
    let mut z1 = low[2];

    let dz0 = if top[1] == low[1] {
        0.0
    } else {
        (top[2] - low[2]) / (top[1] - low[1])
    };

    let dz1 = if mid[1] == low[1] {
        0.0
    } else {
        (mid[2] - low[2]) / (mid[1] - low[1])
    };

    // TODO: reuse z_buffer for multiple frames
    let mut z_buffer = ZBuffer::new();

    let mut dz;
    for y in low[1] as i64 .. mid[1] as i64 {
        dz = if x1 == x0 { 0.0 } else { (z1 - z0) / (x1 - x0) };
        flat_line(img, &mut z_buffer, x0 as i64, x1 as i64, y, z0, dz, clr);
        x0 += dx0;
        x1 += dx1;
        z0 += dz0;
        z1 += dz1;
    }

    let mut x2 = mid[0];
    let dx2 = if top[0] == mid[0] {
        0.0
    } else {
        (top[0] - mid[0]) / (top[1] - mid[1])
    };

    let mut z2 = mid[2];
    let dz2 = if top[1] == mid[1] {
        0.0
    } else {
        (top[2] - mid[2]) / (top[1] - mid[1])
    };
    let mut dz;
    for y in mid[1] as i64 .. top[1] as i64 {
        dz = if x0 == x2 { 0.0 } else { (z2 - z0) / (x2 - x0) };
        flat_line(img, &mut z_buffer, x0 as i64, x2 as i64, y, z0, dz, clr);
        x0 += dx0;
        x2 += dx2;
        z0 += dz0;
        z2 += dz2;
    }
}

#[allow(dead_code)]
fn fclamp(min: f64, x: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn flat_line(img: &mut Screen, z_buffer: &mut ZBuffer, mut x0: i64, mut x1: i64, y: i64, given_z0: f64, given_dz: f64, clr: Color) {
    use std::cmp::{ min, max };
    // Return if y is offscreen
    if y < 0 || y >= HEIGHT as i64 {
        return;
    }
    // Order x0 and x1, and z0 and dz (z0 is the z coord at x0)
    let z0;
    let dz;
    if x1 < x0 {
        swap(&mut x0, &mut x1);
        z0 = given_z0 + given_dz * (x1 - x0) as f64;
        dz = -given_dz;
    } else {
        z0 = given_z0;
        dz = given_dz;
    }
    // Increment x1, as the inputs are inclusive and we'll want the upper
    // bound be be exclusive for everything below
    x1 += 1;
    // Redefine variables as usizes and clamp x within the screen
    let x0 = min(max(x0, 0), WIDTH as i64 - 1) as usize;
    let x1 = min(max(x1, 0), WIDTH as i64 - 1) as usize;
    let y = y as usize;
    let mut z = z0;
    for x in x0..x1 {
        if z_buffer.plot(x, y, z) {
            img.setxy(x, y, clr);
            // debug_clr to show Z, for debugging. TODO: remove this comment once it certainly is correct
            // let debug_clr = Color::grayscale(fclamp(0.0, (z + 60.0) / 200.0 * 255.0, 255.0) as u8);
        }
        z += dz;
    }
}

/// Draw a line in `image` using Bresenham's line algorithm (and variants for each octant).
pub fn line(image: &mut Screen, start: Point, end: Point, color: Color) {
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

/// If the point `p` is within the width and height of `image`, plot `color` at `p`.
pub fn plot_if_visible(image: &mut Screen, p: Point, color: Color) {
    let within_x = p.x >= 0 && p.x < WIDTH as i64;
    let within_y = p.y >= 0 && p.y < HEIGHT as i64;
    if within_x && within_y {
        image.setxy(p.x as usize, p.y as usize, color);
    }
}

/// Bresenham's Line Algorithm for octant 1
fn bline_oct1(image: &mut Screen, mut start: Point, end: Point, color: Color) {
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
fn bline_oct2(image: &mut Screen, mut start: Point, end: Point, color: Color) {
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
fn bline_oct7(image: &mut Screen, mut start: Point, end: Point, color: Color) {
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
fn bline_oct8(image: &mut Screen, mut start: Point, end: Point, color: Color) {
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
