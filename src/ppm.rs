use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc::Receiver;

use render::Color;
use consts::*;

pub fn save_ppm(image: &Vec<Vec<Color>>, filename: &str) {
    let path = Path::new(filename);
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
    write_image(&mut file, &image);
}

pub fn spawn_saver(rx: Receiver<(String, Box<Vec<Vec<Color>>>)>) {
    thread::spawn(move || {
        for (name, screen) in rx {
            save_png(&screen, &name);
        }
    });
}

pub fn save_png(image: &Vec<Vec<Color>>, filename: &str) {
    save_ppm(image, ".temp.ppm");
    let status0 = Command::new("convert")
        .arg(".temp.ppm")
        .arg(filename)
        .status().ok().unwrap();
    println!("Execution of `convert .temp.ppm {}` exited with status: {}", filename, status0);
}

pub fn clean_up() {
    let status1 = Command::new("rm")
        .arg(".temp.ppm")
        .status().ok().unwrap();
    println!("Execution of `rm .temp.ppm` exited with status: {}", status1);
}

#[allow(dead_code)]
pub fn display_file(filename: &str) {
    let status = Command::new("display")
        .arg(filename)
        .status().ok().unwrap();
    println!("Execution of `display {}` exited with status: {}", filename, status);
}

pub fn display_image(image: &Vec<Vec<Color>>) {
    save_png(image, ".temp.png");
    let status0 = Command::new("display")
        .arg(".temp.png")
        .status().ok().unwrap();
    println!("Execution of `display .temp.png` exited with status: {}", status0);
    let status1 = Command::new("rm")
        .arg(".temp.png")
        .status().ok().unwrap();
    println!("Execution of `rm .temp.ppm` exited with status: {}", status1);
}

pub fn write_header(file: &mut File, width: usize, height: usize) {
    if let Err(reason) = write!(file, "P3\n{} {} 255\n", width, height) {
        panic!("could not write header to file. Error: {}",
               reason.description());
    }
}

pub fn write_image(file: &mut File, image: &Vec<Vec<Color>>) {
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
