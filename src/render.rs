use matrix::Matrix;
use point::Point;
use point::Color;
use line::Line;

pub fn plot(image: &mut Vec<Vec<Color>>, p: Point, clr: Color) {
    image[p.y][p.x] = clr;
}

/// Draw edges in an edge list matrix. Each successive pair of
/// elements are considered the endpoints of a distinct edge.
///
/// All edges are drawn in white.
pub fn edge_list(image: &mut Vec<Vec<Color>>, edges: Matrix) {
    let mut c = 0;
    while c + 1 < edges.width() {
        let p = edges.col(c);
        let q = edges.col(c + 1);
        let l = Line::xyxy(
            p[0] as usize,
            p[1] as usize,
            q[0] as usize,
            q[1] as usize);
        line(image, l, Color::white());
        c += 2;
    }
}

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

fn within_screen(p: Point, image: &mut Vec<Vec<Color>>) -> bool {
    // Values are unsigned, so >= 0 is redundant
    let within_y = p.y < image.len();
    let within_x = image.len() > 0 && p.x < image[0].len();
    within_y && within_x
}

fn bline_oct1(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy - dx;
    let mut here: Point = line.startpoint();
    while here.x <= line.x1 && within_screen(here, image) {
        plot(image, here, clr);
        here.x += 1;
        d += dy;
        if d > 0 {
            here.y += 1;
            d -= dx;
        }
    }
}

fn bline_oct2(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy - dx;
    let mut here: Point = line.startpoint();
    let a = dx;
    let b = -dy;
    while here.y <= line.y1 && within_screen(here, image) {
        plot(image, here, clr);
        if d > 0 {
            here.x += 1;
            d += b;
        }
        here.y += 1;
        d += a;
    }
}

pub fn bline_oct7(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = dy + 2 * dx;
    let mut here: Point = line.startpoint();
    let b = -2 * dx;
    let a = 2 * dy;
    while here.y >= line.y1 && within_screen(here, image) {
        plot(image, here, clr);
        if d > 0 {
            here.x += 1;
            d += a;
        }
        // Special check for y == 0 to protect from overflow error in non-release binaries
        if here.y == 0 {
            break;
        }
        here.y -= 1;
        d -= b;
    }
}

fn bline_oct8(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy + dx;
    let mut here: Point = line.startpoint();
    let a = 2 * dy;
    let b = -2 * dx;
    while here.x <= line.x1 {
        plot(image, here, clr);
        if d < 0 {
            // Special check for y == 0 to protect from overflow error in non-release binaries
            if here.y == 0 {
                break;
            }
            here.y -= 1;
            d -= b;
        }
        here.x += 1;
        d += a;
    }
}
