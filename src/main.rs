/// Matrix math
mod matrix;

/// Add curves to an edge matrix
mod curve;

/// Add 3D solids to an edge matrix
mod solid;

/// Render edges to an in-memory representation of the pixels of the screen
mod render;

/// Create image files
mod ppm;

/// Execute commands from a script
mod exec;

/// Crate-wide constants
mod consts;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    match File::open("script") {
        Err(e) => {
            panic!("Could not open file 'script'. Error: {}", e);
        },
        Ok(mut file) => {
            let mut s = String::from("");
            match file.read_to_string(&mut s) {
                Ok(_) => {
                    if let Err(msg) = exec::run_script(&s) {
                        println!("Error!\n{}", msg);
                    }
                },
                Err(e) => {
                    panic!("Error reading text in ./script: {}", e);
                }
            }
        }
    }
}
