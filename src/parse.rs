use render::Color;

// TODO: Use Result instead of panics for error handling
// The error handling here and in mod exec is a mess.

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z
}

#[derive(Clone, Debug)]
pub struct Variation<'a> {
    pub knob: &'a str,
    pub fst_frame: usize,
    pub last_frame: usize,
    pub min_val: f64,
    pub max_val: f64,
}

// TODO: find better place for LightingConstants struct

#[derive(Clone, Debug)]
pub struct LightingConstants {
    ka_r: f64,
    kd_r: f64,
    ks_r: f64,
    ka_g: f64,
    kd_g: f64,
    ks_g: f64,
    ka_b: f64,
    kd_b: f64,
    ks_b: f64,
    // TODO: what's up with R, G, B "intensities" (optional args described in MDL.spec)
}

#[derive(Debug)]
pub enum Command<'a> {
    Push,
    Pop,
    Save(&'a str),
    Display,
    Move { x: f64, y: f64, z: f64, knob: Option<&'a str> },
    Rotate(Axis, f64, Option<&'a str>),
    Scale { x: f64, y: f64, z: f64, knob: Option<&'a str> },
    Box { x: f64, y: f64, z: f64, w: f64, h: f64, d: f64 }, // TODO: add Option<...>s for cs and constants
    Sphere { x: f64, y: f64, z: f64, r: f64 },
    Torus { x: f64, y: f64, z: f64, r0: f64, r1: f64 },
    Line { x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64 },
    Frames(usize),
    Basename(&'a str),
    Vary(Variation<'a>),
    Set(&'a str, f64),
    SetKnobs(f64),
    Ambient(f64, f64, f64), // r, g, b
    Light(f64, f64, f64, f64, f64, f64), // r, g, b, x, y, z
    Constants(&'a str, LightingConstants),
}

pub fn parse<'a>(script: &'a str) -> Result<Vec<Command<'a>>, &'static str> {
    let mut cmds = vec![];

    for mut line in script.lines() {
        skip_linespace(&mut line);
        // Skip blank lines and comments
        // TODO: handle comments at end of lines with commands
        // TODO: make this not a jank one-liner
        if line.chars().nth(0) == Some('#') ||
                line.len() == 0 ||
                (line.chars().nth(0) == Some('/') &&
                 line.chars().nth(1) == Some('/')) {
            continue;
        }

        let command = match next_lexeme(&mut line)? {
            "push" => Command::Push,

            "pop" => Command::Pop,

            "save" => {
                let filename = next_lexeme(&mut line)?;
                Command::Save(filename)
            },

            "display" => Command::Display,

            "move" => {
                Command::Move {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line),
                    knob: next_lexeme(&mut line).ok()
                }
            },

            "rotate" => {
                Command::Rotate(
                    next_axis(&mut line),
                    next_float(&mut line),
                    next_lexeme(&mut line).ok())
            },

            "scale" => {
                Command::Scale {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line),
                    knob: next_lexeme(&mut line).ok()
                }
            },

            "box" => {
                Command::Box {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line),
                    w: next_float(&mut line),
                    h: next_float(&mut line),
                    d: next_float(&mut line)
                }
            },

            "sphere" => {
                Command::Sphere {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line),
                    r: next_float(&mut line),
                }
            },

            "torus" => {
                Command::Torus {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line),
                    r0: next_float(&mut line),
                    r1: next_float(&mut line),
                }
            },

            "line" => {
                Command::Line {
                    x0: next_float(&mut line),
                    y0: next_float(&mut line),
                    z0: next_float(&mut line),
                    x1: next_float(&mut line),
                    y1: next_float(&mut line),
                    z1: next_float(&mut line)
                }
            },

            "frames" => Command::Frames(next_usize(&mut line)),

            "basename" => Command::Basename(next_lexeme(&mut line)?),

            "vary" => {
                Command::Vary(Variation {
                    knob: next_lexeme(&mut line)?,
                    fst_frame: next_usize(&mut line),
                    last_frame: next_usize(&mut line),
                    min_val: next_float(&mut line),
                    max_val: next_float(&mut line)
                })
            },

            "set" => Command::Set(next_lexeme(&mut line)?, next_float(&mut line)),

            "setknobs" => Command::SetKnobs(next_float(&mut line)),

            "ambient" => {
                Command::Ambient(next_float(&mut line), next_float(&mut line), next_float(&mut line))
            },

            "light" => {
                Command::Light(
                    next_float(&mut line),
                    next_float(&mut line),
                    next_float(&mut line),
                    next_float(&mut line),
                    next_float(&mut line),
                    next_float(&mut line))
            },

            "constants" => {
                Command::Constants(
                    next_lexeme(&mut line)?,
                    LightingConstants {
                        ka_r: next_float(&mut line),
                        kd_r: next_float(&mut line),
                        ks_r: next_float(&mut line),
                        ka_g: next_float(&mut line),
                        kd_g: next_float(&mut line),
                        ks_g: next_float(&mut line),
                        ka_b: next_float(&mut line),
                        kd_b: next_float(&mut line),
                        ks_b: next_float(&mut line),
                    }
                )
            },

            other => {
                panic!("Error! Unknown command '{}'!", other);
            }
        };
        cmds.push(command);
        // TODO: error on extra input
    }
    Ok(cmds)
}

fn skip_linespace<'a, 'b>(src: &'b mut &'a str) {
    for (i, c) in src.char_indices() {
        // Plow through src until we hit a newline or non-linespace char
        if c == '\n' || (c != ' ' && c != '\t') {
            // Assign src to the slice after the whitespace
            *src = src.split_at(i).1;
            return;
        }
    }
    *src = ""; // no input after whitespace
}

fn next_lexeme<'a, 'b>(src: &'b mut &'a str) -> Result<&'a str, &'static str> {
    skip_linespace(src);
    if src.len() == 0 {
        // TODO: take metadata as a parameter to have better error messages
        Err("Unexpected end of line in script!")
    } else {
        for (i, c) in src.char_indices() {
            if c.is_whitespace() {
                let (lexeme, rest) = src.split_at(i);
                *src = rest;
                return Ok(lexeme);
            }
        }
        // If the loop finished, all of src is one lexeme
        let lexeme = *src;
        *src = ""; // No input after lexeme
        Ok(lexeme)
    }
}

// TODO: return a Result
fn next_usize(srcref: &mut &str) -> usize {
    if let Ok(lexeme) = next_lexeme(srcref) {
        match lexeme.parse::<usize>() {
            Ok(x) => x,
            Err(_) => panic!("Error! Expected floating point number, found {}", lexeme)
        }
    } else {
        panic!("Error! Expected floating point number, found end of line");
    }
}

// TODO: return a Result
fn next_float(srcref: &mut &str) -> f64 {
    if let Ok(lexeme) = next_lexeme(srcref) {
        match lexeme.parse::<f64>() {
            Ok(x) => x,
            Err(_) => panic!("Error! Expected floating point number, found {}", lexeme)
        }
    } else {
        panic!("Error! Expected floating point number, found end of line");
    }
}

fn next_axis(srcref: &mut &str) -> Axis {
    let lexeme = next_lexeme(srcref);
    match lexeme {
        Ok("x") => Axis::X,
        Ok("y") => Axis::Y,
        Ok("z") => Axis::Z,
        Ok(word) => panic!("Error! Expected floating point number, found {}", word),
        Err(_) => panic!("Error! Expected floating points number, found end of line")
    }
}
