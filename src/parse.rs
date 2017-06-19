use std::fmt;
use exec::{ Variation, LightingConstants };

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub enum ParseError<'a> {
    // If the token field (the first field) is None, EOL (or EOF) was found.
    // The second field (usize) is line number.
    ExpectedNumber(Option<&'a str>, usize),
    ExpectedUnsigned(Option<&'a str>, usize),
    ExpectedName(Option<&'a str>, usize),
    ExpectedAxis(Option<&'a str>, usize),
    UnexpectedTrailing(&'a str, usize),
    UnknownCommand(&'a str, usize),
}

// Util function for Display impl for ParseError. Displays bad tokens or unexpected end of line
fn err_display_eol_or_lexeme(optlex: Option<&str>) -> String {
    if let Some(lexeme) = optlex {
        format!("'{}'", lexeme)
    } else {
        format!("end of line")
    }
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error on line ")?;
        // Then print the line number and the error info
        match *self {
            ParseError::ExpectedNumber(optlex, linum) => {
                write!(f, "{}. Expected number, found {}", linum, err_display_eol_or_lexeme(optlex))
            },
            ParseError::ExpectedUnsigned(optlex, linum) => {
                write!(f, "{}. Expected unsigned integer, found {}", linum, err_display_eol_or_lexeme(optlex))
            },
            ParseError::ExpectedName(optlex, linum) => {
                write!(f, "{}. Expected name, found {}", linum, err_display_eol_or_lexeme(optlex))
            },
            ParseError::ExpectedAxis(optlex, linum) => {
                write!(f, "{}. Expected axis (x or y or z), found {}", linum, err_display_eol_or_lexeme(optlex))
            },
            ParseError::UnexpectedTrailing(trailing, linum) => {
                write!(f, "{}. Unexpected characters after command: '{}'", linum, trailing)
            },
            ParseError::UnknownCommand(name, linum) => {
                write!(f, "{}. Unknown command '{}'", linum, name)
            }
        }
    }
}

pub fn parse_script<'a>(script: &'a str) -> Result<Vec<Command<'a>>, ParseError<'a>> {
    let mut cmds = Vec::new();
    for (linum, mut line) in script.lines().enumerate() {
        // Skip blank lines and comments
        skip_linespace(&mut line);
        if line.len() == 0 || starts_with_comment(line) {
            continue;
        }

        let cmd = next_cmd(&mut line, linum)?;

        cmds.push(cmd);

        // Check for trailing input
        skip_linespace(&mut line);
        if !starts_with_comment(line) && line != "" {
            return Err(ParseError::UnexpectedTrailing(line, linum));
        }
    }
    Ok(cmds)
}

fn skip_linespace<'a>(src: &mut &'a str) {
    for (i, c) in src.char_indices() {
        // Plow through src until we hit a newline or non-whitespace char
        if c == '\n' || !c.is_whitespace() {
            // Assign src to the slice after the whitespace
            *src = src.split_at(i).1;
            return;
        }
    }
    *src = ""; // no input after whitespace
}

fn starts_with_comment(src: &str) -> bool {
    src.chars().nth(0) == Some('/') &&
        src.chars().nth(1) == Some('/')
}

fn next_cmd<'a>(lineref: &mut &'a str, linum: usize) -> Result<Command<'a>, ParseError<'a>> {
    let cmd = match next_name(lineref, linum)? {
        "push" => Command::Push,

        "pop" => Command::Pop,

        "save" => {
            let filename = next_name(lineref, linum)?;
            Command::Save(filename)
        },

        "display" => Command::Display,

        "move" => {
            Command::Move {
                x: next_float(lineref, linum)?,
                y: next_float(lineref, linum)?,
                z: next_float(lineref, linum)?,
                knob: next_name(lineref, linum).ok()
            }
        },

        "rotate" => {
            Command::Rotate(
                next_axis(lineref, linum)?,
                next_float(lineref, linum)?,
                next_name(lineref, linum).ok())
        },

        "scale" => {
            Command::Scale {
                x: next_float(lineref, linum)?,
                y: next_float(lineref, linum)?,
                z: next_float(lineref, linum)?,
                knob: next_name(lineref, linum).ok()
            }
        },

        "box" => {
            Command::Box {
                x: next_float(lineref, linum)?,
                y: next_float(lineref, linum)?,
                z: next_float(lineref, linum)?,
                w: next_float(lineref, linum)?,
                h: next_float(lineref, linum)?,
                d: next_float(lineref, linum)?
            }
        },

        "sphere" => {
            Command::Sphere {
                x: next_float(lineref, linum)?,
                y: next_float(lineref, linum)?,
                z: next_float(lineref, linum)?,
                r: next_float(lineref, linum)?,
            }
        },

        "torus" => {
            Command::Torus {
                x: next_float(lineref, linum)?,
                y: next_float(lineref, linum)?,
                z: next_float(lineref, linum)?,
                r0: next_float(lineref, linum)?,
                r1: next_float(lineref, linum)?,
            }
        },

        "line" => {
            Command::Line {
                x0: next_float(lineref, linum)?,
                y0: next_float(lineref, linum)?,
                z0: next_float(lineref, linum)?,
                x1: next_float(lineref, linum)?,
                y1: next_float(lineref, linum)?,
                z1: next_float(lineref, linum)?
            }
        },

        "frames" => Command::Frames(next_usize(lineref, linum)?),

        "basename" => Command::Basename(next_name(lineref, linum)?),

        "vary" => {
            Command::Vary(Variation {
                knob: next_name(lineref, linum)?,
                fst_frame: next_usize(lineref, linum)?,
                last_frame: next_usize(lineref, linum)?,
                min_val: next_float(lineref, linum)?,
                max_val: next_float(lineref, linum)?
            })
        },

        "set" => Command::Set(next_name(lineref, linum)?, next_float(lineref, linum)?),

        "setknobs" => Command::SetKnobs(next_float(lineref, linum)?),

        "ambient" => {
            Command::Ambient(next_float(lineref, linum)?, next_float(lineref, linum)?, next_float(lineref, linum)?)
        },

        "light" => {
            Command::Light(
                next_float(lineref, linum)?,
                next_float(lineref, linum)?,
                next_float(lineref, linum)?,
                next_float(lineref, linum)?,
                next_float(lineref, linum)?,
                next_float(lineref, linum)?)
        },

        "constants" => {
            Command::Constants(
                next_name(lineref, linum)?,
                LightingConstants {
                    ka_r: next_float(lineref,  linum)?,
                    kd_r: next_float(lineref,  linum)?,
                    ks_r: next_float(lineref,  linum)?,
                    ka_g: next_float(lineref,  linum)?,
                    kd_g: next_float(lineref,  linum)?,
                    ks_g: next_float(lineref,  linum)?,
                    ka_b: next_float(lineref,  linum)?,
                    kd_b: next_float(lineref,  linum)?,
                    ks_b: next_float(lineref,  linum)?,
                }
                )
        },

        other => {
            return Err(ParseError::UnknownCommand(other, linum));
        }
    };

    Ok(cmd)
}

/// Return value:
/// - Some(*first lexeme in `src`*)
/// - None if there is no lexeme in `src` (i.e. there is only whitespace)
/// Lexemes are sequences on non-whitespace chars on the same line
fn next_lexeme<'a>(src: &mut &'a str) -> Option<&'a str> {
    skip_linespace(src);
    if src.len() == 0 {
        None
    } else {
        for (i, c) in src.char_indices() {
            if c.is_whitespace() {
                let (lexeme, rest) = src.split_at(i);
                *src = rest;
                return Some(lexeme);
            }
        }
        // If the loop finished, all of src is one lexeme
        let lexeme = *src;
        *src = ""; // No input after lexeme
        Some(lexeme)
    }
}

fn next_name<'a>(srcref: &mut &'a str, i: usize) -> Result<&'a str, ParseError<'a>> {
    if let Some(lexeme) = next_lexeme(srcref) {
        let mut chars = lexeme.chars();
        // Match for one alphabetic char, then alphanumeric chars and underscores
        if chars.next().unwrap().is_alphabetic() {
            // `unwrap` because lexeme always has at least one character
            for c in chars {
                if !c.is_alphabetic() && !c.is_digit(10) && c != '_' {
                    return Err(ParseError::ExpectedName(Some(lexeme), i));
                }
            }
        }
        Ok(lexeme)
    } else {
        Err(ParseError::ExpectedName(None, i))
    }
}

fn next_usize<'a>(srcref: &mut &'a str, i: usize) -> Result<usize, ParseError<'a>> {
    if let Some(lexeme) = next_lexeme(srcref) {
        match lexeme.parse::<usize>() {
            Ok(x) => Ok(x),
            Err(_) => Err(ParseError::ExpectedUnsigned(Some(lexeme), i))
        }
    } else {
        Err(ParseError::ExpectedUnsigned(None, i))
    }
}

fn next_float<'a>(srcref: &mut &'a str, i: usize) -> Result<f64, ParseError<'a>> {
    if let Some(lexeme) = next_lexeme(srcref) {
        match lexeme.parse::<f64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(ParseError::ExpectedNumber(Some(lexeme), i))
        }
    } else {
        Err(ParseError::ExpectedNumber(None, i))
    }
}

fn next_axis<'a>(srcref: &mut &'a str, i: usize) -> Result<Axis, ParseError<'a>> {
    let lexeme = next_lexeme(srcref);
    match lexeme {
        Some("x") => Ok(Axis::X),
        Some("y") => Ok(Axis::Y),
        Some("z") => Ok(Axis::Z),
        Some(word) => Err(ParseError::ExpectedAxis(Some(word), i)),
        None => Err(ParseError::ExpectedAxis(None, i)),
    }
}
