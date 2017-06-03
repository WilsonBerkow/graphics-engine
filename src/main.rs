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

mod parse;

/// Execute commands from a script
mod exec;

mod worker;

/// Crate-wide constants
mod consts;

use std::fs::File;

use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::time::Instant;

fn main() {
    match File::open("script") {
        Err(e) => {
            panic!("Could not open file 'script'. Error: {}", e);
        },
        Ok(mut file) => {
            let mut s = String::from("");
            match file.read_to_string(&mut s) {
                Ok(_) => {
                    let (tx, rx) = channel();
                    let handle = ppm::spawn_saver(rx);
                    ppm::mkdirp("anim");
                    let start = Instant::now();
                    let frame_info: Option<(usize, &str)>;
                    match exec::run_script(&s, tx) {
                        Err(msg) => {
                            println!("Error!\n{}", msg);
                            frame_info = None;
                        },
                        Ok(opt_frame_info) => {
                            frame_info = opt_frame_info;
                        }
                    }
                    handle.join();
                    let elapsed = start.elapsed();
                    println!("Total time: {}s {}ms", elapsed.as_secs(), elapsed.subsec_nanos() as u64 / 1000000);
                    if let Some((frames, basename)) = frame_info {
                        ppm::clean_up_anim_ppms(frames, basename);
                    }
                },
                Err(e) => {
                    panic!("Error reading text in ./script: {}", e);
                }
            }
        }
    }
}
