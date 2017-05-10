use parse::{ self, Command, Axis };
use matrix::Matrix;
use solid;
use render;
use ppm;
use consts::*;

pub fn run_script(script: &str) -> Result<(), String> {
    let mut screen = vec![vec![render::Color::black(); WIDTH]; HEIGHT];
    let mut transforms = vec![Matrix::identity()];

    for cmd in parse::parse(script) {
        run_cmd(&mut screen, &mut transforms, cmd)?;
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

fn run_cmd(screen: &mut Vec<Vec<render::Color>>, transforms: &mut Vec<Matrix>, cmd: Command) -> Result<(), String> {
    match cmd {
        Command::Line { x0, y0, z0, x1, y1, z1 } => {
            let mut edges = Matrix::empty();
            edges.push_edge(
                [x0, y0, z0, 1.0],
                [x1, y1, z1, 1.0]);
            edges = last(&transforms) * &edges;
            render::edge_list(screen, &edges);
            Ok(())
        },

        // TODO: (Parse and) draw curves as well. It was not assigned, but good to have.

        Command::Box { x, y, z, w, h, d } => {
            let mut triangles = Matrix::empty();
            solid::rect_prism(&mut triangles, x, y, z, w, h, d);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, &triangles);
            Ok(())
        },

        Command::Sphere { x, y, z, r } => {
            let mut triangles = Matrix::empty();
            solid::sphere(&mut triangles, x, y, z, r);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, &triangles);
            Ok(())
        },

        Command::Torus { x, y, z, r0, r1 } => {
            let mut triangles = Matrix::empty();
            solid::torus(&mut triangles, x, y, z, r0, r1);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, &triangles);
            Ok(())
        },

        Command::Push => {
            let top = last(&transforms).clone();
            transforms.push(top);
            Ok(())
        },

        Command::Pop => {
            transforms.pop();
            Ok(())
        },

        Command::Scale { x, y, z } => {
            transform_last(&Matrix::dilation_xyz(x, y, z), transforms);
            Ok(())
        },

        Command::Move { x, y, z } => {
            transform_last(&Matrix::translation_xyz(x, y, z), transforms);
            Ok(())
        },

        Command::Rotate(axis, angle) => {
            let rotation = match axis {
                Axis::X => Matrix::rotation_about_x(angle),
                Axis::Y => Matrix::rotation_about_y(angle),
                Axis::Z => Matrix::rotation_about_z(angle)
            };
            transform_last(&rotation, transforms);
            Ok(())
        },

        Command::Display => {
            ppm::display_image(&screen);
            Ok(())
        },

        Command::Save(name) => {
            ppm::save_png(&screen, name);
            Ok(())
        }
    }
}

