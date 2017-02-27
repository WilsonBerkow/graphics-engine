use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use point::Color;
use consts::*;

pub fn make_ppm<T>(f: T) where T: FnOnce(&mut Vec<Vec<Color>>) {
    let mut image = vec![vec![Color::rgb(0, 0, 50); WIDTH]; HEIGHT];
    f(&mut image);
    let image = image; // No longer mutable

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

    // write image to file
    write_image(&mut file, &image);
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
