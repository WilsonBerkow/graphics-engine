use matrix::Matrix;
use curve;
use solid;
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
    let mut screen = vec![vec![render::Color::black(); WIDTH]; HEIGHT];
    let mut transforms = vec![Matrix::identity()];

    let mut toks = script.split_whitespace();
    while let Some(cmd) = toks.next() {
        run_cmd(&mut screen, &mut transforms, cmd, &mut toks)?;
    }
    Ok(())
}

fn last<T>(v: &Vec<T>) -> &T {
    &v[v.len() - 1]
}

fn transform_last(mat: &Matrix, transforms: &mut Vec<Matrix>) {
    let len = transforms.len();
    transforms[len - 1].transform_on_right(mat);
}

fn run_cmd(screen: &mut Vec<Vec<render::Color>>, transforms: &mut Vec<Matrix>, cmd: &str, toks: &mut SplitWhitespace) -> Result<(), String> {
    match cmd {
        "line" => {
            let mut edges = Matrix::empty();
            let x0 = next_num(toks, cmd)?;
            let y0 = next_num(toks, cmd)?;
            let z0 = next_num(toks, cmd)?;
            let x1 = next_num(toks, cmd)?;
            let y1 = next_num(toks, cmd)?;
            let z1 = next_num(toks, cmd)?;
            edges.push_edge(
                [x0, y0, z0, 1.0],
                [x1, y1, z1, 1.0]);
            edges = last(&transforms) * &edges;
            render::edge_list(screen, &edges);
            Ok(())
        },

        "circle" => {
            let mut edges = Matrix::empty();
            let cx = next_num(toks, cmd)?;
            let cy = next_num(toks, cmd)?;
            let cz = next_num(toks, cmd)?;
            let r = next_num(toks, cmd)?;
            curve::circle(&mut edges, cx, cy, cz, r);
            edges = last(&transforms) * &edges;
            render::edge_list(screen, &edges);
            Ok(())
        },

        "hermite" => {
            let mut edges = Matrix::empty();
            let x0 = next_num(toks, cmd)?;
            let y0 = next_num(toks, cmd)?;
            let x1 = next_num(toks, cmd)?;
            let y1 = next_num(toks, cmd)?;
            let xm0 = next_num(toks, cmd)?;
            let ym0 = next_num(toks, cmd)?;
            let xm1 = next_num(toks, cmd)?;
            let ym1 = next_num(toks, cmd)?;
            curve::hermite(&mut edges, 128,
                           [x0, y0, 0.0, 1.0],
                           [x1, y1, 0.0, 1.0],
                           [xm0, ym0, 0.0, 1.0],
                           [xm1, ym1, 0.0, 1.0]);
            edges = last(&transforms) * &edges;
            render::edge_list(screen, &edges);
            Ok(())
        },

        "bezier" => {
            let mut edges = Matrix::empty();
            let x0 = next_num(toks, cmd)?;
            let y0 = next_num(toks, cmd)?;
            let x1 = next_num(toks, cmd)?;
            let y1 = next_num(toks, cmd)?;
            let x2 = next_num(toks, cmd)?;
            let y2 = next_num(toks, cmd)?;
            let x3 = next_num(toks, cmd)?;
            let y3 = next_num(toks, cmd)?;
            curve::bezier(&mut edges, 128,
                          [x0, y0, 0.0, 1.0],
                          [x1, y1, 0.0, 1.0],
                          [x2, y2, 0.0, 1.0],
                          [x3, y3, 0.0, 1.0]);
            edges = last(&transforms) * &edges;
            render::edge_list(screen, &edges);
            Ok(())
        },

        "box" => {
            let mut triangles = Matrix::empty();
            let x = next_num(toks, cmd)?;
            let y = next_num(toks, cmd)?;
            let z = next_num(toks, cmd)?;
            let dx = next_num(toks, cmd)?;
            let dy = next_num(toks, cmd)?;
            let dz = next_num(toks, cmd)?;
            solid::rect_prism(&mut triangles, x, y, z, dx, dy, dz);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, &triangles);
            Ok(())
        },

        "sphere" => {
            let mut triangles = Matrix::empty();
            let cx = next_num(toks, cmd)?;
            let cy = next_num(toks, cmd)?;
            let cz = next_num(toks, cmd)?;
            let r = next_num(toks, cmd)?;
            solid::sphere(&mut triangles, cx, cy, cz, r);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, &triangles);
            Ok(())
        },

        "torus" => {
            let mut triangles = Matrix::empty();
            let cx = next_num(toks, cmd)?;
            let cy = next_num(toks, cmd)?;
            let cz = next_num(toks, cmd)?;
            let big_radius = next_num(toks, cmd)?;
            let lil_radius = next_num(toks, cmd)?;
            solid::torus(&mut triangles, cx, cy, cz, big_radius, lil_radius);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, &triangles);
            Ok(())
        },

        "push" => {
            let top = last(&transforms).clone();
            transforms.push(top);
            Ok(())
        },

        "pop" => {
            transforms.pop();
            Ok(())
        },

        "scale" => {
            let sx = next_num(toks, cmd)?;
            let sy = next_num(toks, cmd)?;
            let sz = next_num(toks, cmd)?;
            transform_last(&Matrix::dilation_xyz(sx, sy, sz), transforms);
            Ok(())
        },

        "move" => {
            let dx = next_num(toks, cmd)?;
            let dy = next_num(toks, cmd)?;
            let dz = next_num(toks, cmd)?;
            transform_last(&Matrix::translation_xyz(dx, dy, dz), transforms);
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
                transform_last(&rotation, transforms);
                Ok(())
            } else {
                Err("Unexpected end of file after \"rotate\"".to_owned())
            }
        },

        "display" => {
            ppm::display_image(&screen);
            Ok(())
        },

        "save" => {
            if let Some(name) = toks.next() {
                ppm::save_png(&screen, name);
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

