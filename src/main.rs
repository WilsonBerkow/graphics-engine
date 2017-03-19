/// Matrix math
mod matrix;

/// Add curves to an edge matrix
mod curve;

/// Render edges to an in-memory representation of the pixels of the screen
mod render;

/// Create image files
mod ppm;

/// Parse commands from a script (custom language used for this class)
mod parse;

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
                    let tokens = parse::parse_tokens(&s);
                    exec::run_script(tokens);
                },
                Err(e) => {
                    panic!("Error reading text in ./script: {}", e);
                }
            }
        }
    }
}

