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
    let mut image = vec![vec![Color::rgb(0, 0, 0); WIDTH]; HEIGHT];

    let ch1 = Point::new(WIDTH * 3 / 8, CY); // negative charge
    let ch2 = Point::new(WIDTH * 5 / 8, CY); // positive charge
    let ch3 = Point::new(WIDTH * 7 / 8, HEIGHT / 4); // negative charge

    for px in 0..WIDTH {
        for py in 0..HEIGHT {
            let p = Point::new(px, py);
            let magnitude = 250000.0 * (-p.dist_to(ch1).powi(-2) + p.dist_to(ch2).powi(-2) - p.dist_to(ch3).powi(-2)).abs();
            let lightness = if magnitude > 255.0 { 255 } else { magnitude as u8 };
            // Another way of calculating lightness, which assumes we do not take abs() of
            // magnitude, and discriminates on magnitude's sign:
            // let strength = if magnitude > 127.0 { 127 } else if magnitude < -128.0 { -128 } else { magnitude as i8 };
            // let lightness = ((strength as i16) + 128) as u8;
            image[py][px] = Color::rgb(lightness, lightness, lightness);
        }
    }

    // draw charge of ch1
    fill_crect(&mut image, ch1.x + 1, ch1.y, 20, 4, Color::rgb(255, 0, 0));
    // draw charge of ch2
    fill_crect(&mut image, ch2.x + 1, ch2.y, 20, 4, Color::rgb(0, 0, 255));
    fill_crect(&mut image, ch2.x + 1, ch2.y, 4, 20, Color::rgb(0, 0, 255));
    // draw charge of ch3
    fill_crect(&mut image, ch3.x + 1, ch3.y, 20, 4, Color::rgb(255, 0, 0));

    // write image to file
    write_image(&mut file, &image);
}

fn fill_crect(image: &mut Vec<Vec<Color>>, x: usize, y: usize, w: usize, h: usize, clr: Color) {
    fill_rect(image, x - w / 2, y - h / 2, w, h, clr);
}

fn fill_rect(image: &mut Vec<Vec<Color>>, x: usize, y: usize, w: usize, h: usize, clr: Color) {
    for px in x..x + w {
        for py in y..y + h {
            image[py][px] = clr;
        }
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
    fn new(x: usize, y: usize) -> Point {
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
