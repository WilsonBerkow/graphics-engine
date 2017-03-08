mod matrix;
mod point;
mod line;
mod render;
mod ppm;
mod consts;
mod work2;
mod work3;

use matrix::Matrix;
use std::fs::File;
use std::io::prelude::*;

use consts::*;

#[derive(Debug, Clone, Copy)]
enum Command {
    Line,
    Ident,
    Scale,
    Move,
    Rotate,
    Apply,
    Display,
    Save
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z
}

#[derive(Debug, Clone)]
enum Token {
    Cmd(Command),
    Num(f64),
    Axis(Axis),
    FileName(String),
}

fn unwrap_num(t: &Token) -> f64 {
    if let &Token::Num(x) = t {
        return x;
    } else {
        panic!("parse error: expected number; found {:?}", t);
    }
}

fn unwrap_axis(t: &Token) -> Axis {
    if let &Token::Axis(axis) = t {
        return axis;
    } else {
        panic!("parse error: expected x or y or z; found {:?}", t);
    }
}

fn parse_tokens(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    for word in s.split_whitespace() {
        toks.push(parse_token(word));
    }
    toks
}

fn parse_token(s: &str) -> Token {
    match s {
        "line" => Token::Cmd(Command::Line),
        "ident" => Token::Cmd(Command::Ident),
        "scale" => Token::Cmd(Command::Scale),
        "move" => Token::Cmd(Command::Move),
        "rotate" => Token::Cmd(Command::Rotate),
        "apply" => Token::Cmd(Command::Apply),
        "display" => Token::Cmd(Command::Display),
        "save" => Token::Cmd(Command::Save),
        "x" => Token::Axis(Axis::X),
        "y" => Token::Axis(Axis::Y),
        "z" => Token::Axis(Axis::Z),
        s => {
            if let Ok(x) = s.parse() {
                Token::Num(x)
            } else {
                Token::FileName(s.to_owned())
            }
        }
    }
}

fn run_script(toks: Vec<Token>) {
    let mut edges = Matrix::empty();
    let mut transform = Matrix::identity();

    let mut i = 0;
    while i < toks.len() {
        match toks[i] {
            Token::Cmd(Command::Line) => {
                let x0 = unwrap_num(&toks[i + 1]);
                let y0 = unwrap_num(&toks[i + 2]);
                let z0 = unwrap_num(&toks[i + 3]);
                let x1 = unwrap_num(&toks[i + 4]);
                let y1 = unwrap_num(&toks[i + 5]);
                let z1 = unwrap_num(&toks[i + 6]);
                edges.push_edge(
                    [x0, y0, z0, 1.0],
                    [x1, y1, z1, 1.0]);
                i += 6;
            },
            Token::Cmd(Command::Ident) => {
                transform = Matrix::identity();
                i += 1;
            },
            Token::Cmd(Command::Scale) => {
                let sx = unwrap_num(&toks[i + 1]);
                let sy = unwrap_num(&toks[i + 2]);
                let sz = unwrap_num(&toks[i + 3]);
                transform = &Matrix::dilation_xyz(sx, sy, sz) * &transform;
                i += 3;
            },
            Token::Cmd(Command::Move) => {
                let dx = unwrap_num(&toks[i + 1]);
                let dy = unwrap_num(&toks[i + 2]);
                let dz = unwrap_num(&toks[i + 3]);
                transform = &Matrix::translation_xyz(dx, dy, dz) * &transform;
                i += 3;
            },
            Token::Cmd(Command::Rotate) => {
                let axis = unwrap_axis(&toks[i + 1]);
                let angle = unwrap_num(&toks[i + 2]).to_radians();
                let rot = match axis {
                    Axis::X => Matrix::rotation_about_x(angle),
                    Axis::Y => Matrix::rotation_about_y(angle),
                    Axis::Z => Matrix::rotation_about_z(angle)
                };
                transform = &rot * &transform;
                i += 3;
            },
            Token::Cmd(Command::Apply) => {
                edges = &transform * &edges;
                i += 1;
            },
            Token::Cmd(Command::Display) => {
                use point::Color;
                use ppm;
                let mut image = vec![vec![Color::black(); WIDTH]; HEIGHT];
                render::edge_list(&mut image, &edges);
                ppm::display_image(&image);
                i += 1;
            },
            Token::Cmd(Command::Save) => {
                if let &Token::FileName(ref name) = &toks[i + 1] {
                    use point::Color;
                    use ppm;
                    use render;
                    let mut image = vec![vec![Color::rgb(0, 0, 0); WIDTH]; HEIGHT];
                    render::edge_list(&mut image, &edges);
                    ppm::save_png(&image, name);
                    i += 2;
                } else {
                    panic!("Expected filename; found {:?}", &toks[i + 1]);
                }
            },
            _ => {i += 1;}
        }
    }
}

fn main() {
    match File::open("script") {
        Err(e) => {
            panic!("Could not open file 'script'. Error: {}", e);
        },
        Ok(mut file) => {
            let mut s = String::from("");
            match file.read_to_string(&mut s) {
                Ok(_) => {
                    let toks = parse_tokens(&s);
                    println!("{:?}", toks);
                    run_script(toks);
                },
                Err(e) => {
                    panic!("Error reading text in ./script: {}", e);
                }
            }
        }
    }
}
