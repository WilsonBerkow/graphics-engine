use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod point;
mod line;
mod render;

use point::{Color, Point};
use line::Line;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const CX: usize = WIDTH / 2;
const CY: usize = HEIGHT / 2;

fn main() {
    let path = Path::new("img.ppm");
    let path_display = path.display(); // For safe string formatting
    let mut file = match File::create(&path) {
        Err(reason) => {
            panic!("could not create {}. Error: {}",
                   path_display,
                   reason.description())
        }
        Ok(file) => file,
    };
    write_header(&mut file, WIDTH, HEIGHT);
    let mut image = vec![vec![Color::rgb(0, 0, 50); WIDTH]; HEIGHT];

    for i in 0..(HEIGHT / 20) {
        // down-right
        render::line(&mut image,
             Line::xyxy(0, i * 1, WIDTH - 1, i * 19),
             Color::white());
        // down-left
        render::line(&mut image,
             Line::xyxy(WIDTH - 1, i * 1, 0, i * 19),
             Color::white());
        // up-right
        render::line(&mut image,
             Line::xyxy(0, HEIGHT - 1 - i * 1, WIDTH - 1, HEIGHT - 1 - i * 19),
             Color::white());
        // up-left
        render::line(&mut image,
             Line::xyxy(WIDTH - 1, HEIGHT - 1 - i * 1, 0, HEIGHT - 1 - i * 19),
             Color::white());
    }

    // write image to file
    write_image(&mut file, &image);
}

fn write_header(file: &mut File, width: usize, height: usize) {
    if let Err(reason) = write!(file, "P3\n{} {} 255\n", width, height) {
        panic!("could not write header to file. Error: {}",
               reason.description());
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
        panic!("could not write image to file. Error: {}",
               reason.description());
    }
}
