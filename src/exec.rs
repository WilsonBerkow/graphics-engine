use std::time::Instant;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use parse::{ self, Command, Axis };
use matrix::Matrix;
use solid;
use render::{ self, Screen, ZBuffer };
use ppm;
use consts::*;

// TODO: clean up w/ regard to distinction between single-image and animation rendering
// Ok-component of return value is None if only a static frame was generated, and Some((frames,
// basename)) if an animation was made.
pub fn run_script(script: &str, tx: Sender<(String, Screen)>) -> Result<Option<(usize, &str)>, String> {
    let cmds = parse::parse(script)?;

    let mut z_buffer = ZBuffer::new();

    match get_anim_data(&cmds) {
        Some(anim_data) => {
            if DEBUG {
                println!("anim_data: {:?}", &anim_data);
            }

            let basename = anim_data.basename.unwrap_or("anim");

            // Render and save each frame:
            for i in 0..anim_data.frames {
                if i != 0 {
                    z_buffer.clear();
                }
                let mut screen = Screen::new();
                let start = Instant::now();
                let mut knobvals = knobs_for_frame(i, &anim_data.varies);
                let mut transforms = vec![Matrix::identity()];
                screen.clear_black();
                for cmd in &cmds {
                    run_cmd(&mut screen, &mut z_buffer, &mut transforms, Some(&mut knobvals), cmd)?;
                }
                if DEBUG {
                    let elapsed = start.elapsed();
                    println!("Took: {}", elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1000000);
                }
                let filename = anim_frame_filename(anim_data.frames, basename, i);
                tx.send((filename, screen));
            }
            Ok(Some((anim_data.frames, basename)))
        },
        None => {
            let mut screen = Screen::new();
            let mut transforms = vec![Matrix::identity()];
            for cmd in &cmds {
                run_cmd(&mut screen, &mut z_buffer, &mut transforms, None, cmd)?;
            }
            Ok(None)
        }
    }
}

// Generate the filename used for the nth frame of an animation
pub fn anim_frame_filename(frames: usize, basename: &str, n: usize) -> String {
    let digits_for_name = decimal_digits(frames);
    format!("anim/{}{:0digits$}.png", basename, n, digits=digits_for_name)
}

fn decimal_digits(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

#[derive(Debug)]
struct AnimData<'a> {
    frames: usize,
    basename: Option<&'a str>,
    varies: Vec<parse::Variation<'a>>
}

fn get_anim_data<'a>(commands: &Vec<Command<'a>>) -> Option<AnimData<'a>> {
    let mut mframes = None;
    let mut mbasename = None;
    let mut varies = vec![];
    for cmd in commands {
        match cmd {
            &Command::Frames(f) => {
                mframes = Some(f);
            },
            &Command::Basename(s) => {
                mbasename = Some(s);
            },
            &Command::Vary(ref variation) => {
                varies.push(variation.clone());
            },
            _ => {}
        }
    }
    if let Some(frames) = mframes {
        return Some(AnimData {
            frames: frames,
            basename: mbasename,
            varies: varies
        });
    }
    if varies.len() > 0 {
        if DEBUG {
            println!("WARNING: found 'vary' but not 'frames'");
        }
    }
    return None;
}

fn knob_val<'a>(knobs: &HashMap<&'a str, f64>, knob: &'a str) -> f64 {
    match knobs.get(knob) {
        Some(v) => *v,
        None => {
            panic!("Knob '{}' not defined for every frame", knob);
        },
    }
}

fn optknob_val<'a>(optknobs: Option<&HashMap<&'a str, f64>>, optknob: Option<&'a str>) -> f64 {
    if let (Some(knobs), Some(knob)) = (optknobs, optknob) {
        knob_val(knobs, knob)
    } else {
        1.0 // default to 1.0 to not scale values at all
    }
}

fn knobs_for_frame<'a>(frame: usize, varies: &Vec<parse::Variation<'a>>) -> HashMap<&'a str, f64> {
    let mut knob_vals = vec![];
    for vary in varies {
        if vary.fst_frame <= frame && frame <= vary.last_frame {
            // // FIXME: doing this with binary search is cute but may well
            // // be slower than just using linear search and appending to the end
            // // Insert the knob-value association unless there already is one for this knob
            // match knob_vals.binary_search_by_key(vary.knob, |v| v.knob) {
            //     Ok(pos) => {
            //         // There is already a val for this knob
            //         panic!("ERROR: Overlapping vary commands for knob '{}'", vary.knob);
            //     },
            //     Err(pos) => {
            //         // The knob is not yet in the list
            //         knob_vals.insert(pos, ...);
            //         // Yes, this is O(n). See FIXME above.
            //     }
            // }
            let progress = (frame - vary.fst_frame) as f64 / (vary.last_frame - vary.fst_frame) as f64;
            let val = vary.min_val + (vary.max_val - vary.min_val) * progress;
            knob_vals.push((vary.knob, val))
        }
        // Otherwise, this 'vary' doesn't apply to the current frame.
    }
    return knob_vals.into_iter().collect();
}

fn last<T>(v: &Vec<T>) -> &T {
    &v[v.len() - 1]
}

fn transform_last(mat: &Matrix, transforms: &mut Vec<Matrix>) {
    let len = transforms.len();
    transforms[len - 1] = &transforms[len - 1] * mat;
}

fn run_cmd<'a>(screen: &mut Screen, z_buffer: &mut ZBuffer, transforms: &mut Vec<Matrix>, knobs: Option<&mut HashMap<&'a str, f64>>, cmd: &Command<'a>) -> Result<(), String> {
    match cmd {
        &Command::Line { x0, y0, z0, x1, y1, z1 } => {
            let mut edges = Matrix::empty();
            edges.push_edge(
                [x0, y0, z0, 1.0],
                [x1, y1, z1, 1.0]);
            edges = last(&transforms) * &edges;
            render::edge_list(screen, &edges);
            Ok(())
        },

        // TODO: (Parse and) draw curves as well. It was not assigned, but is nice to have.

        &Command::Box { x, y, z, w, h, d } => {
            let mut triangles = Matrix::empty();
            solid::rect_prism(&mut triangles, x, y, z, w, h, d);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, z_buffer, &triangles);
            Ok(())
        },

        &Command::Sphere { x, y, z, r } => {
            let mut triangles = Matrix::empty();
            solid::sphere(&mut triangles, x, y, z, r);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, z_buffer, &triangles);
            Ok(())
        },

        &Command::Torus { x, y, z, r0, r1 } => {
            let mut triangles = Matrix::empty();
            solid::torus(&mut triangles, x, y, z, r0, r1);
            triangles = last(&transforms) * &triangles;
            render::triangle_list(screen, z_buffer, &triangles);
            Ok(())
        },

        &Command::Push => {
            let top = last(&transforms).clone();
            transforms.push(top);
            Ok(())
        },

        &Command::Pop => {
            transforms.pop();
            Ok(())
        },

        &Command::Scale { x, y, z, knob } => {
            let t = optknob_val(knobs.map(|x| &*x), knob);
            transform_last(&Matrix::dilation_xyz(t * x, t * y, t * z), transforms);
            Ok(())
        },

        &Command::Move { x, y, z, knob } => {
            let t = optknob_val(knobs.map(|x| &*x), knob);
            transform_last(&Matrix::translation_xyz(t * x, t * y, t * z), transforms);
            Ok(())
        },

        &Command::Rotate(axis, degrees, knob) => {
            let t = optknob_val(knobs.map(|x| &*x), knob);
            let radians = degrees.to_radians();
            let rotation = match axis {
                Axis::X => Matrix::rotation_about_x(t * radians),
                Axis::Y => Matrix::rotation_about_y(t * radians),
                Axis::Z => Matrix::rotation_about_z(t * radians)
            };
            transform_last(&rotation, transforms);
            Ok(())
        },

        &Command::Display => {
            ppm::display_image(&screen);
            Ok(())
        },

        &Command::Save(name) => {
            ppm::save_png(&screen, name);
            Ok(())
        },

        &Command::Set(knob, val) => {
            match knobs {
                Some(knobs) => {
                    let r = knobs.entry(knob).or_insert(0.0);
                    *r = val;
                },
                None => {
                    // TODO: instead of None, just have an empty HashMap so set can be used
                },
            }
            Ok(())
        },

        &Command::SetKnobs(v) => {
            match knobs {
                Some(knobs) => {
                    for val in knobs.values_mut() {
                        *val = v;
                    }
                },
                None => {
                    // TODO: instead of None, just have an empty HashMap so set can be used
                },
            }
            Ok(())
        },

        &Command::Frames(..) | &Command::Basename(..) | &Command::Vary { .. } => {
            Ok(())
        }
    }
}

