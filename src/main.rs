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

use consts::*;

fn main() {
    // TODO: flatten structure of `match`es here
    match File::open("script") {
        Err(e) => {
            panic!("Could not open file 'script'. Error: {}", e);
        },
        Ok(mut file) => {
            let mut s = String::from("");
            match file.read_to_string(&mut s) {
                Ok(_) => {
                    // `tx` is sent to run_script, which generates frames (on the heap) and
                    // sends them to `tx`. `rx` is given to spawn_saver which duplicates it
                    // into several worker threads for saving frames to a file.
                    let (tx, rx) = channel();
                    let handle = ppm::spawn_saver(rx);
                    println!("Using {} worker threads to save and convert frames", NUM_WORKERS);

                    // Make `anim` directory unless it exists
                    ppm::mkdirp("anim");

                    // Generate frames:
                    let start = Instant::now();

                    // Parse script and exit on syntax error
                    let cmds = match parse::parse(&s) {
                        Ok(cmds) => cmds,
                        Err(parse_error) => {
                            println!("\n{}", parse_error);
                            return;
                        }
                    };

                    // frame_info, if Some, is a pair of the number of frames and the basename, and
                    // is used to delete intermediate files (e.g. .ppm files) at the end.
                    let frame_info: Option<(usize, &str)>;
                    match exec::run_script(&cmds, tx) {
                        Err(msg) => {
                            println!("Error!\n{}", msg);
                            frame_info = None;
                        },
                        Ok(opt_frame_info) => {
                            frame_info = opt_frame_info;
                            let elapsed = start.elapsed();
                            println!("Time to generate frames in-memory: {} (includes some time saving images)", display_duration(elapsed));
                        }
                    }
                    // Wait for worker threads to finish saving images
                    handle.join();

                    let elapsed = start.elapsed();
                    println!("Elapsed time, after generating frames and converting to PNGs: {}", display_duration(elapsed));

                    // If (multiple) frames were successfully generated, make a GIF and delete the rubbish
                    if let Some((frames, basename)) = frame_info {
                        ppm::convert_gif(frames, basename);
                        ppm::clean_up_anim_ppms(frames, basename);
                    }

                    let elapsed_after_cleanup = start.elapsed();
                    println!("Elapsed time, after cleaning up and converting to GIF: {}", display_duration(elapsed_after_cleanup));
                },
                Err(e) => {
                    panic!("Error reading text in ./script: {}", e);
                }
            }
        }
    }
}

fn display_duration(elapsed: std::time::Duration) -> String {
    format!("{}s {}ms", elapsed.as_secs(), elapsed.subsec_nanos() as u64 / 1000000)
}
