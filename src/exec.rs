use matrix::Matrix;
use curve;
use render;
use ppm;
use consts::*;
use std;
use std::str::SplitWhitespace;

fn next_num<'a, T: std::iter::Iterator<Item=&'a str>>(iter: &mut T, cmd: &str) -> Result<f64, String> {
    if let Some(tok) = iter.next() {
        if let Ok(num) = tok.parse::<f64>() {
            Ok(num)
        } else {
            Err(format!("Expected numeric argument to {}, found '{}'", cmd, tok))
        }
    } else {
        Err(format!("Unexpected end of file; expected numeric argument to {}", cmd))
    }
}


pub fn run_script(script: &str) -> Result<(), String> {
    let mut edges = Matrix::empty();
    let mut transform = Matrix::identity();

    let mut toks = script.split_whitespace();
    while let Some(cmd) = toks.next() {
        run_cmd(&mut edges, &mut transform, cmd, &mut toks)?;
    }
    Ok(())
}

fn run_cmd(edges: &mut Matrix, transform: &mut Matrix, cmd: &str, toks: &mut SplitWhitespace) -> Result<(), String> {
    match cmd {
        "clear" => {
            edges.clear_cols();
            Ok(())
        },

        "line" => {
            let x0 = next_num(toks, cmd)?;
            let y0 = next_num(toks, cmd)?;
            let z0 = next_num(toks, cmd)?;
            let x1 = next_num(toks, cmd)?;
            let y1 = next_num(toks, cmd)?;
            let z1 = next_num(toks, cmd)?;
            edges.push_edge(
                [x0, y0, z0, 1.0],
                [x1, y1, z1, 1.0]);
            Ok(())
        },

        "circle" => {
            let cx = next_num(toks, cmd)?;
            let cy = next_num(toks, cmd)?;
            let cz = next_num(toks, cmd)?;
            let r = next_num(toks, cmd)?;
            curve::circle(edges, cx, cy, cz, r);
            Ok(())
        },

        "hermite" => {
            let x0 = next_num(toks, cmd)?;
            let y0 = next_num(toks, cmd)?;
            let x1 = next_num(toks, cmd)?;
            let y1 = next_num(toks, cmd)?;
            let xm0 = next_num(toks, cmd)?;
            let ym0 = next_num(toks, cmd)?;
            let xm1 = next_num(toks, cmd)?;
            let ym1 = next_num(toks, cmd)?;
            curve::hermite(edges, 128,
                           [x0, y0, 0.0, 1.0],
                           [x1, y1, 0.0, 1.0],
                           [xm0, ym0, 0.0, 1.0],
                           [xm1, ym1, 0.0, 1.0]);
            Ok(())
        },

        "bezier" => {
            let x0 = next_num(toks, cmd)?;
            let y0 = next_num(toks, cmd)?;
            let x1 = next_num(toks, cmd)?;
            let y1 = next_num(toks, cmd)?;
            let x2 = next_num(toks, cmd)?;
            let y2 = next_num(toks, cmd)?;
            let x3 = next_num(toks, cmd)?;
            let y3 = next_num(toks, cmd)?;
            curve::bezier(edges, 128,
                          [x0, y0, 0.0, 1.0],
                          [x1, y1, 0.0, 1.0],
                          [x2, y2, 0.0, 1.0],
                          [x3, y3, 0.0, 1.0]);
            Ok(())
        },

        "ident" => {
            *transform = Matrix::identity();
            Ok(())
        },

        "scale" => {
            let sx = next_num(toks, cmd)?;
            let sy = next_num(toks, cmd)?;
            let sz = next_num(toks, cmd)?;
            transform.transform_by(&Matrix::dilation_xyz(sx, sy, sz));
            Ok(())
        },

        "move" => {
            let dx = next_num(toks, cmd)?;
            let dy = next_num(toks, cmd)?;
            let dz = next_num(toks, cmd)?;
            transform.transform_by(&Matrix::translation_xyz(dx, dy, dz));
            Ok(())
        },

        "rotate" => {
            if let Some(axis) = toks.next() {
                let angle = next_num(toks, cmd)?.to_radians();
                let rotation = match axis {
                    "x" => Matrix::rotation_about_x(angle),
                    "y" => Matrix::rotation_about_y(angle),
                    "z" => Matrix::rotation_about_z(angle),
                    _ => {
                        return Err(format!("Expected x or y or z, found {}", axis));
                    }
                };
                transform.transform_by(&rotation);
                Ok(())
            } else {
                Err("Unexpected end of file after \"rotate\"".to_owned())
            }
        },

        "apply" => {
            edges.transform_by(transform);
            Ok(())
        },

        "display" => {
            let mut image = vec![vec![render::Color::black(); WIDTH]; HEIGHT];
            render::edge_list(&mut image, &edges);
            ppm::display_image(&image);
            Ok(())
        },

        "save" => {
            if let Some(name) = toks.next() {
                let mut image = vec![vec![render::Color::black(); WIDTH]; HEIGHT];
                render::edge_list(&mut image, &edges);
                ppm::save_png(&image, name);
                Ok(())
            } else {
                Err("Unexpected end of file after \"save\"".to_owned())
            }
        },

        ref s => {
            panic!("Unexpected token {}", s);
        }
    }
}

