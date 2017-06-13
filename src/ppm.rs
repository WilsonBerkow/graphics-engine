use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::Command;

use std::sync::mpsc::Receiver;
use std::time::Instant;

use worker::WorkerPool;
use render::Screen;
use consts::*;

pub fn save_ppm(image: &Screen, filename: &str) {
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

pub fn spawn_saver(rx: Receiver<(String, Screen)>) -> WorkerPool {
    WorkerPool::new(rx, NUM_WORKERS)
}

pub fn save_png(image: &Screen, filename: &str) {
    let tmp_name = format!("{}.ppm", filename);
    save_ppm(image, &tmp_name);
    let start = Instant::now();
    let status = Command::new("convert")
        .arg(&tmp_name)
        .arg(filename)
        .status().expect("failed to execute convert command");
    if DEBUG {
        let elapsed = start.elapsed();
        println!("Convert took: {}ms", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1000000);
    }
    if !status.success() {
        println!("Execution of `convert {} {}` failed with status: {}", &tmp_name, filename, status);
    }
}

pub fn mkdirp(name: &str) {
    let status = Command::new("mkdir")
        .arg("-p")
        .arg("--")
        .arg(name)
        .status().expect("failed to execute mkdir command");
    if !status.success() {
        println!("Execution of `mkdir {}` failed with status: {}", name, status);
    }
}

pub fn convert_gif(frames: usize, basename: &str) {
    use exec::anim_frame_filename;
    let mut args = Vec::with_capacity(frames + 1);
    for i in 0..frames {
        let name = anim_frame_filename(frames, basename, i);
        args.push(name);
    }
    let gif_path = format!("anim/{}.gif", basename);
    args.push(gif_path.clone());

    // Remove the gif file
    let status0 = Command::new("rm")
        .arg("-f")
        .arg("--")
        .arg(&gif_path)
        .status().expect("failed to execute rm command");
    if !status0.success() {
        println!("Execution of `rm anim/{}.gif` failed with status0: {}", basename, status0);
    }

    let status1 = Command::new("convert")
        .args(&*args.into_boxed_slice())
        .status().expect("failed to execute convert command");
    if !status1.success() {
        println!("Execution of `convert [... frames ...] {}.gif` failed with status1: {}", basename, status1);
    }
}

pub fn clean_up_anim_ppms(frames: usize, basename: &str) {
    use exec::anim_frame_filename;
    for i in 0..frames {
        let filename = format!("{}.ppm", anim_frame_filename(frames, basename, i));
        let status = Command::new("rm")
            .arg("--")
            .arg(&filename)
            .status().expect("failed to execute rm command");
        if !status.success() {
            println!("Execution of `rm {}` failed with status: {}", &filename, status);
        }
    }
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

pub fn display_image(image: &Screen) {
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

pub fn write_image(file: &mut File, image: &Screen) {
    let start = Instant::now();
    let mut bufwriter = BufWriter::new(&*file);
    // P6 identifies the version of PPM in which colors are represented
    // in binary; as our max color value is 255, each RGB color is 3 bytes
    bufwriter.write_fmt(format_args!("P6\n{} {} 255\n", WIDTH, HEIGHT));
    bufwriter.write(image.as_bytes());
    if DEBUG {
        let elapsed = start.elapsed();
        println!("Saving took: {}ms {}ns", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1000000, elapsed.subsec_nanos() as u64 % 1000000);
    }
}
