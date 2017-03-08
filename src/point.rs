use std;
use std::fmt::Display;

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

    pub fn fmt_ppm(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    pub fn dist_to(&self, p: Point) -> f64 {
        let x1 = self.x as f64;
        let y1 = self.y as f64;
        let x2 = p.x as f64;
        let y2 = p.y as f64;
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

