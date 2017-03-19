use matrix::Matrix;
use curve;
use parse::{Token, Command, Axis};
use render;
use ppm;
use consts::*;

fn unwrap_num(t: &Token) -> f64 {
    if let &Token::Num(x) = t {
        return x;
    } else {
        panic!("parse error: expected number; found {:?}", t);
    }
}

fn next_num(t: &Vec<Token>, i: &mut usize) -> f64 {
    if let Token::Num(x) = t[*i] {
        *i += 1;
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

pub fn run_script(toks: Vec<Token>) {
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
                i += 7;
            },

            Token::Cmd(Command::Circle) => {
                i += 1;
                let cx = next_num(&toks, &mut i);
                let cy = next_num(&toks, &mut i);
                let cz = next_num(&toks, &mut i);
                let r = next_num(&toks, &mut i);
                curve::circle(&mut edges, cx, cy, cz, r);
            },

            Token::Cmd(Command::Hermite) => {
                i += 1;
                let x0 = next_num(&toks, &mut i);
                let y0 = next_num(&toks, &mut i);
                let x1 = next_num(&toks, &mut i);
                let y1 = next_num(&toks, &mut i);
                let xm0 = next_num(&toks, &mut i);
                let ym0 = next_num(&toks, &mut i);
                let xm1 = next_num(&toks, &mut i);
                let ym1 = next_num(&toks, &mut i);
                curve::hermite(&mut edges, 128,
                    [x0, y0, 0.0, 1.0],
                    [x1, y1, 0.0, 1.0],
                    [xm0, ym0, 0.0, 1.0],
                    [xm1, ym1, 0.0, 1.0]);
            },

            Token::Cmd(Command::Bezier) => {
                i += 1;
                let x0 = next_num(&toks, &mut i);
                let y0 = next_num(&toks, &mut i);
                let x1 = next_num(&toks, &mut i);
                let y1 = next_num(&toks, &mut i);
                let x2 = next_num(&toks, &mut i);
                let y2 = next_num(&toks, &mut i);
                let x3 = next_num(&toks, &mut i);
                let y3 = next_num(&toks, &mut i);
                curve::bezier(&mut edges, 128,
                    [x0, y0, 0.0, 1.0],
                    [x1, y1, 0.0, 1.0],
                    [x2, y2, 0.0, 1.0],
                    [x3, y3, 0.0, 1.0]);
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
                i += 4;
            },

            Token::Cmd(Command::Move) => {
                let dx = unwrap_num(&toks[i + 1]);
                let dy = unwrap_num(&toks[i + 2]);
                let dz = unwrap_num(&toks[i + 3]);
                transform = &Matrix::translation_xyz(dx, dy, dz) * &transform;
                i += 4;
            },

            Token::Cmd(Command::Rotate) => {
                let axis = unwrap_axis(&toks[i + 1]);
                let angle = unwrap_num(&toks[i + 2]).to_radians();
                let rotation = match axis {
                    Axis::X => Matrix::rotation_about_x(angle),
                    Axis::Y => Matrix::rotation_about_y(angle),
                    Axis::Z => Matrix::rotation_about_z(angle)
                };
                transform = &rotation * &transform;
                i += 3;
            },

            Token::Cmd(Command::Apply) => {
                edges = &transform * &edges;
                i += 1;
            },

            Token::Cmd(Command::Display) => {
                let mut image = vec![vec![render::Color::black(); WIDTH]; HEIGHT];
                render::edge_list(&mut image, &edges);
                ppm::display_image(&image);
                i += 1;
            },

            Token::Cmd(Command::Save) => {
                if let &Token::FileName(ref name) = &toks[i + 1] {
                    let mut image = vec![vec![render::Color::black(); WIDTH]; HEIGHT];
                    render::edge_list(&mut image, &edges);
                    ppm::save_png(&image, name);
                    i += 2;
                } else {
                    panic!("Expected filename; found {:?}", &toks[i + 1]);
                }
            },
            ref t => {
                panic!("Unexpected token {:?} (token number {})", t, i);
            }
        }
    }
}

