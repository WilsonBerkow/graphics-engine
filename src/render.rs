use matrix::Matrix;
use point::Point;
use point::Color;
use line::Line;

/// Draw edges in an edge list matrix. Each successive pair of
/// elements are considered the endpoints of a distinct edge.
///
/// All edges are drawn in white.
pub fn edge_list(image: &mut Vec<Vec<Color>>, edges: &Matrix) {
    let mut c = 0;
    while c + 1 < edges.width() {
        let p = edges.col(c);
        let q = edges.col(c + 1);
        let l = Line::xyxy(
            p[0] as i64,
            p[1] as i64,
            q[0] as i64,
            q[1] as i64);
        line(image, l, Color::white());
        c += 2;
    }
}

/// Draw a line in `image` using Bresenham's line algorithm (and variants for each octant).
pub fn line(image: &mut Vec<Vec<Color>>, mut line: Line, clr: Color) {
    if line.x0 > line.x1 {
        line = line.reversed();
    }
    let more_vertical = line.dy().abs() > line.dx().abs();
    if line.y1 > line.y0 {
        if more_vertical {
            bline_oct2(image, line, clr);
        } else {
            bline_oct1(image, line, clr);
        }
    } else {
        if more_vertical {
            bline_oct7(image, line, clr);
        } else {
            bline_oct8(image, line, clr);
        }
    }
}

fn within_screen(image: &mut Vec<Vec<Color>>, p: Point) -> bool {
    let within_y = p.y >= 0 && p.y < image.len() as i64;
    let within_x = p.x >= 0 && image.len() > 0 && p.x < image[0].len() as i64;
    within_y && within_x
}

/// If the point `p` is within the width and height of `image`, plot `clr` at `p`.
pub fn plot_if_visible(image: &mut Vec<Vec<Color>>, p: Point, clr: Color) {
    if within_screen(image, p) {
        image[p.y as usize][p.x as usize] = clr;
    }
}

/// Bresenham's Line Algorithm for octant 1
fn bline_oct1(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy - dx;
    let mut here: Point = line.startpoint();
    while here.x <= line.x1 {
        plot_if_visible(image, here, clr);
        here.x += 1;
        d += dy;
        if d > 0 {
            here.y += 1;
            d -= dx;
        }
    }
}

/// Bresenham's Line Algorithm for octant 2
fn bline_oct2(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy - dx;
    let mut here: Point = line.startpoint();
    let a = dx;
    let b = -dy;
    while here.y <= line.y1 {
        plot_if_visible(image, here, clr);
        if d > 0 {
            here.x += 1;
            d += b;
        }
        here.y += 1;
        d += a;
    }
}

/// Bresenham's Line Algorithm for octant 7
pub fn bline_oct7(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = dy + 2 * dx;
    let mut here: Point = line.startpoint();
    let b = -2 * dx;
    let a = 2 * dy;
    while here.y >= line.y1 {
        plot_if_visible(image, here, clr);
        if d > 0 {
            here.x += 1;
            d += a;
        }
        here.y -= 1;
        d -= b;
    }
}

/// Bresenham's Line Algorithm for octant 8
fn bline_oct8(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy + dx;
    let mut here: Point = line.startpoint();
    let a = 2 * dy;
    let b = -2 * dx;
    while here.x <= line.x1 {
        plot_if_visible(image, here, clr);
        if d < 0 {
            here.y -= 1;
            d -= b;
        }
        here.x += 1;
        d += a;
    }
}
