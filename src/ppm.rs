use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::Command;

use std::sync::mpsc::Receiver;
use std::time::Instant;

use worker::WorkerPool;
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
    write_image(&mut file, &image);
}

pub fn spawn_saver(rx: Receiver<(String, Box<Vec<Vec<Color>>>)>) -> WorkerPool {
    WorkerPool::new(rx, 8)
}

pub fn save_png(image: &Vec<Vec<Color>>, filename: &str) {
    let tmp_name = format!("{}.ppm", filename);
    save_ppm(image, &tmp_name);
    let start = Instant::now();
    let status0 = Command::new("convert")
        .arg(&tmp_name)
        .arg(filename)
        .status().ok().unwrap();
    let elapsed = start.elapsed();
    if DEBUG { println!("Convert took: {}ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1000000); }
    println!("Execution of `convert {} {}` exited with status: {}", &tmp_name, filename, status0);
}

pub fn clean_up() {
    // TODO
    //let status1 = Command::new("rm")
    //    .arg("*.ppm")
    //    .status().ok().unwrap();
    //println!("Execution of `rm .temp.ppm` exited with status: {}", status1);
}

#[allow(dead_code)]
pub fn display_file(filename: &str) {
    let status = Command::new("display")
        .arg(filename)
        .status().ok().unwrap();
    println!("Execution of `display {}` exited with status: {}", filename, status);
}

// Calculate images in sequence, generating strings and sending strings to be written
// to worker threads

// Also do imagemagick converts in a smarter way (i.e. not every single time)

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

pub fn write_image(file: &mut File, image: &Vec<Vec<Color>>) {
    let start = Instant::now();
    let mut bufwriter = BufWriter::new(&*file);
    // P6 identifies the version of PPM in which colors are represented
    // in binary; as our max color value is 255, each RGB color is 3 bytes
    bufwriter.write_fmt(format_args!("P6\n{} {} 255\n", WIDTH, HEIGHT));
    for py in 0..HEIGHT {
        for px in 0..WIDTH {
            bufwriter.write(&[image[py][px].r, image[py][px].g, image[py][px].b]);
        }
    }
    let elapsed = start.elapsed();
    if DEBUG {
        println!("Saving took: {}ms {}ns", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1000000, elapsed.subsec_nanos() as u64 % 1000000);
    }
}
