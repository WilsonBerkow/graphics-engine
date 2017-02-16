use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fmt::Display;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const CX: usize = WIDTH / 2;
const CY: usize = HEIGHT / 2;

fn main() {
    let path = Path::new("img.ppm");
    let path_display = path.display(); // For safe string formatting
    let mut file = match File::create(&path) {
        Err(reason) => panic!("could not create {}. Error: {}", path_display, reason.description()),
        Ok(file) => file,
    };
    write_header(&mut file, WIDTH, HEIGHT);
    let mut image = vec![vec![Color::rgb(0, 0, 50); WIDTH]; HEIGHT];

    for i in 0..(HEIGHT / 20) {
        // down-right
        line(&mut image, Line::xyxy(0, i * 1, WIDTH - 1, i * 19), Color::white());
        // down-left
        line(&mut image, Line::xyxy(WIDTH - 1, i * 1, 0, i * 19), Color::white());
        // up-right
        line(&mut image, Line::xyxy(0, HEIGHT - 1 - i * 1, WIDTH - 1, HEIGHT - 1 - i * 19), Color::white());
        // up-left
        line(&mut image, Line::xyxy(WIDTH - 1, HEIGHT - 1 - i * 1, 0, HEIGHT - 1 - i * 19), Color::white());
    }

    // write image to file
    write_image(&mut file, &image);
}

fn plot(image: &mut Vec<Vec<Color>>, p: Point, clr: Color) {
    image[p.y][p.x] = clr;
}

fn line(image: &mut Vec<Vec<Color>>, mut line: Line, clr: Color) {
    let ltr = line.x0 < line.x1;
    if !ltr {
        line = line.reversed();
    }
    let more_up = line.y1 + line.x0 > line.x1 + line.y0; // true when dy > dx
    if line.y1 > line.y0 {
        if more_up {
            bline_oct2(image, line, clr);
        } else {
            bline_oct1(image, line, clr);
        }
    } else {
        if more_up {
            bline_oct7(image, line, clr);
        } else {
            bline_oct8(image, line, clr);
        }
    }
}

fn bline_oct1(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = 2 * dy - dx;
    let mut here: Point = line.startpoint();
    while here.x <= line.x1 {
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
    while here.y <= line.y1 {
        plot(image, here, clr);
        if d > 0 {
            here.x += 1;
            d += b;
        }
        here.y += 1;
        d += a;
    }
}

fn bline_oct7(image: &mut Vec<Vec<Color>>, line: Line, clr: Color) {
    let dx: i64 = line.dx();
    let dy: i64 = line.dy();
    let mut d: i64 = dy + 2 * dx;
    let mut here: Point = line.startpoint();
    let a = 2 * dy;
    let b = -2 * dx;
    while here.y >= line.y1 {
        plot(image, here, clr);
        if d > 0 {
            here.x += 1;
            d += a;
        }
        here.y -= 1;
        d -= a;
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
            here.y -= 1;
            d -= b;
        }
        here.x += 1;
        d += a;
    }
}

fn write_header(file: &mut File, width: usize, height: usize) {
    if let Err(reason) = write!(file, "P3\n{} {} 255\n", width, height) {
        panic!("could not write header to file. Error: {}", reason.description());
    }
}

fn write_image(file: &mut File, image: &Vec<Vec<Color>>) {
    let mut contents = String::with_capacity(image.len() * image[0].len());
    for px in 0..WIDTH {
        for py in 0..HEIGHT {
            contents.push_str(&image[px][py].fmt_ppm());
        }
    }
    if let Err(reason) = file.write_all(contents.as_bytes()) {
        panic!("could not write image to file. Error: {}", reason.description());
    }
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }

    fn black() -> Color {
        Color::rgb(0, 0, 0)
    }

    fn white() -> Color {
        Color::rgb(255, 255, 255)
    }

    fn fmt_ppm(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn xy(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    fn dist_to(&self, p: Point) -> f64 {
        let x1 = self.x as f64;
        let y1 = self.y as f64;
        let x2 = p.x as f64;
        let y2 = p.y as f64;
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

#[derive(Clone, Copy)]
struct Line {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Line {
    fn origin_to(x: usize, y: usize) -> Line {
        Line { x0: 0, y0: 0, x1: x, y1: y }
    }

    fn from_to(p0: Point, p1: Point) -> Line {
        Line { x0: p0.x, y0: p0.y, x1: p1.x, y1: p1.y }
    }

    fn xyxy(x0: usize, y0: usize, x1: usize, y1: usize) -> Line {
        Line { x0: x0, y0: y0, x1: x1, y1: y1 }
    }

    fn from_by(x: usize, y: usize, dx: i64, dy: i64) -> Line {
        Line {
            x0: x,
            y0: y,
            x1: (x as i64 + dx) as usize,
            y1: (y as i64 + dy) as usize,
        }
    }

    fn reversed(self) -> Line {
        Line::xyxy(self.x1, self.y1, self.x0, self.y0)
    }

    fn startpoint(self) -> Point {
        Point::xy(self.x0, self.y0)
    }

    fn endpoint(self) -> Point {
        Point::xy(self.x1, self.y1)
    }

    fn dx(self) -> i64 {
        return self.x1 as i64 - self.x0 as i64;
    }

    fn dy(self) -> i64 {
        return self.y1 as i64 - self.y0 as i64;
    }
}
