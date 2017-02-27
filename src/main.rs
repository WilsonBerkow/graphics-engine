use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod matrix;
mod point;
mod line;
mod render;

use matrix::Matrix;
use point::{Color, Point};
use line::Line;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const CX: usize = WIDTH / 2;
const CY: usize = HEIGHT / 2;

fn main() {
    test_matrix();
    test_edge_list();
}

// [Work 3] Print results of some Matrix computations
fn test_matrix() {
    use matrix::Matrix;
    println!("GENERAL MATRIX MATH:\n");
    let m = Matrix::new4x4(
        1.0, 2.0, 3.0, 0.0,
        4.0, 5.0, 6.0, 0.0,
        7.0, 8.0, 9.0, 0.0,
        0.0, 0.0, 0.0, 1.0);
    println!("{}", m);
    println!("{}", Matrix::identity());
    println!("{}", &m * &Matrix::identity());
    println!("{}", &Matrix::identity() * &m);
    let dilate = Matrix::dilation_xyz(1.0, 3.0, 5.0);
    println!("{}", dilate);
    println!("{}", &dilate * &m);
    println!("{}", &dilate * &Matrix::identity());
    println!("============================");
}

// [Work 3] Draw same image from work 2, using Matrix for an edge list.
fn test_edge_list() {
    println!("EDGE MATRIX:\n");
    let mut edges = Matrix::empty();
    for i in 0..(HEIGHT / 20) {
        // down-right lines
        edges.push_edge(
            [0.0, i as f64, 0.0, 1.0],
            [(WIDTH - 1) as f64, (i * 19) as f64, 0.0, 1.0]);
        // down-left lines
        edges.push_edge(
            [(WIDTH - 1) as f64, i as f64, 0.0, 1.0],
            [0.0, (i * 19) as f64, 0.0, 1.0]);
        // up-right lines
        edges.push_edge(
            [0.0, (HEIGHT - 1 - i) as f64, 0.0, 1.0],
            [(WIDTH - 1) as f64, (HEIGHT - 1 - i * 19) as f64, 0.0, 1.0]);
        // up-left lines
        edges.push_edge(
            [(WIDTH - 1) as f64, (HEIGHT - 1 - i) as f64, 0.0, 1.0],
            [0.0, (HEIGHT - 1 - i * 19) as f64, 0.0, 1.0]);
    }
    edges = &Matrix::dilation(0.5) * &edges;
    edges = &Matrix::shear_2d(0.3, 0.8) * &edges;
    edges = &Matrix::rotation_about_z(0.1) * &edges;
    generate_image(|image| {
        render::edge_list(image, edges);
    });
    println!("Saved transformed image to img.ppm");
}

fn generate_image<T>(f: T) where T: FnOnce(&mut Vec<Vec<Color>>) {
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

    f(&mut image);

    // write image to file
    write_image(&mut file, &image);
}

/// work2: Generate pretty line pattern using Bresenham's Line Algorithm (in line.rs).
fn work2() {
    generate_image(|image: &mut Vec<Vec<Color>>| {
        for i in 0..(HEIGHT / 20) {
            // down-right lines
            render::line(
                image,
                Line::xyxy(0, i * 1, WIDTH - 1, i * 19),
                Color::white());
            // down-left lines
            render::line(
                image,
                Line::xyxy(WIDTH - 1, i * 1, 0, i * 19),
                Color::white());
            // up-right lines
            render::line(
                image,
                Line::xyxy(0, HEIGHT - 1 - i * 1, WIDTH - 1, HEIGHT - 1 - i * 19),
                Color::white());
            // up-left lines
            render::line(
                image,
                Line::xyxy(WIDTH - 1, HEIGHT - 1 - i * 1, 0, HEIGHT - 1 - i * 19),
                Color::white());
        }
    });
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
