// TODO: Use Result instead of panics for error handling

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z
}

#[derive(Debug)]
pub enum Command<'a> {
    Push,
    Pop,
    Save(&'a str),
    Display,
    Move { x: f64, y: f64, z: f64 },
    Rotate(Axis, f64),
    Scale { x: f64, y: f64, z: f64 },
    Box { x: f64, y: f64, z: f64, w: f64, h: f64, d: f64 }, // TODO: add Option<...>s for cs and constants
    Sphere { x: f64, y: f64, z: f64, r: f64 },
    Torus { x: f64, y: f64, z: f64, r0: f64, r1: f64 },
    Line { x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64 }
}

pub fn parse<'a>(script: &'a str) -> Vec<Command<'a>> {
    let mut cmds = vec![];
    for mut line in script.lines() {
        skip_linespace(&mut line);
        // Skip blank lines and comments
        if line.chars().nth(0) == Some('#') || line.len() == 0 {
            continue;
        }
        let cmd = next_lexeme(&mut line);

        skip_linespace(&mut line);

        let command = match cmd {
            "push" => Command::Push,
            "pop" => Command::Pop,
            "save" => {
                let filename = next_lexeme(&mut line);
                Command::Save(filename)
            },
            "display" => Command::Display,
            "move" => {
                Command::Move {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line)
                }
            },
            "rotate" => {
                Command::Rotate(
                    next_axis(&mut line),
                    next_float(&mut line).to_radians())
            },
            "scale" => {
                Command::Scale {
                    x: next_float(&mut line),
                    y: next_float(&mut line),
                    z: next_float(&mut line)
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
            other => {
                panic!("Error! Unknown command '{}'!", other);
            }
        };
        cmds.push(command);
        // TODO: error on extra input
    }
    cmds
}

fn skip_linespace<'a, 'b>(src: &'b mut &'a str) {
    for (i, c) in src.char_indices() {
        // Plow through src until we hit a newline or non-linespace char
        if c == '\n' || (c != ' ' && c != '\t') {
            // Assign src to slice after the whitespace
            *src = src.split_at(i).1;
            return;
        }
    }
    *src = ""; // no input after whitespace
}

fn next_lexeme<'a, 'b>(src: &'b mut &'a str) -> &'a str {
    skip_linespace(src);
    if src.len() == 0 {
        panic!("Unexpected end of line in script!");
    }
    for (i, c) in src.char_indices() {
        if c.is_whitespace() {
            let (lexeme, rest) = src.split_at(i);
            *src = rest;
            return lexeme;
        }
    }
    let lexeme = *src;
    *src = ""; // No input after lexeme
    return lexeme;
}

fn next_float(srcref: &mut &str) -> f64 {
    let lexeme = next_lexeme(srcref);
    match lexeme.parse::<f64>() {
        Ok(x) => x,
        Err(_) => panic!("Error! Expected floating point number, found {}", lexeme)
    }
}

fn next_axis(srcref: &mut &str) -> Axis {
    let lexeme = next_lexeme(srcref);
    match lexeme {
        "x" => Axis::X,
        "y" => Axis::Y,
        "z" => Axis::Z,
        _ => panic!("Error! Expected floating point number, found {}", lexeme)
    }
}
