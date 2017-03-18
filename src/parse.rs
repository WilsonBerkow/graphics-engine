#[derive(Debug, Clone, Copy)]
pub enum Command {
    Line,
    Circle,
    Hermite,
    Bezier,
    Ident,
    Scale,
    Move,
    Rotate,
    Apply,
    Display,
    Save
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z
}

#[derive(Debug, Clone)]
pub enum Token {
    Cmd(Command),
    Num(f64),
    Axis(Axis),
    FileName(String),
}

pub fn parse_tokens(s: &str) -> Vec<Token> {
    let mut toks = vec![];
    for word in s.split_whitespace() {
        toks.push(parse_token(word));
    }
    toks
}

pub fn parse_token(s: &str) -> Token {
    match s {
        "line" => Token::Cmd(Command::Line),
        "circle" => Token::Cmd(Command::Circle),
        "hermite" => Token::Cmd(Command::Hermite),
        "bezier" => Token::Cmd(Command::Bezier),
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

