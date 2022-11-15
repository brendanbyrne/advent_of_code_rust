use std::fs::read_to_string;

#[derive(Debug)]
enum ErrorKind {
    EmptyFilo,
    NoLeftovers,
    UnknownCharacter,
    SyntaxError,
}

#[derive(Debug)]
struct ChunkError {
    kind: ErrorKind,
    msg: String,
}

impl ChunkError {
    fn new(kind: ErrorKind, msg: String) -> ChunkError {
        ChunkError {
            kind,
            msg,
        }
    }
}

fn get_score(closed: char) -> Result<u64, ChunkError> {
    match closed {
        ')' => Ok(1),
        ']' => Ok(2),
        '}' => Ok(3),
        '>' => Ok(4),
        _ => Err(ChunkError::new(
            ErrorKind::UnknownCharacter,
            format!("Attempted to score an ungraded character: {closed}"),
        )),
    }
}

fn get_open(closed: char) -> Result<char, ChunkError> {
    match closed {
        ')' => Ok('('),
        ']' => Ok('['),
        '}' => Ok('{'),
        '>' => Ok('<'),
        _ => Err(ChunkError::new(
            ErrorKind::UnknownCharacter,
            format!("No open character for {closed}"),
        )),
    }
}

fn get_closed(closed: char) -> Result<char, ChunkError> {
    match closed {
        '(' => Ok(')'),
        '[' => Ok(']'),
        '{' => Ok('}'),
        '<' => Ok('>'),
        _ => Err(ChunkError::new(
            ErrorKind::UnknownCharacter,
            format!("No open character for {closed}"),
        )),
    }
}

fn is_open(c: char) -> Result<bool, ChunkError> {
    match c {
        '(' | '[' | '{' | '<' => Ok(true),
        ')' | ']' | '}' | '>' => Ok(false),
        _ => Err(ChunkError::new(
            ErrorKind::UnknownCharacter,
            format!("Not an open or closed state for {c}"),
        )),
    }
}

struct Data {
    points: Vec<u64>,
}

fn parse(line: &str) -> Result<u64, ChunkError> {
    let mut filo: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_open(c)? {
            filo.push(c);
        } else if let Some(back) = filo.last() {
            if *back == get_open(c)? {
                filo.pop();
            } else {
                return Err(ChunkError::new(
                    ErrorKind::SyntaxError,
                    format!("Illegal character {c}"),
                ));
            }
        } else {
            return Err(ChunkError::new(
                ErrorKind::EmptyFilo,
                "Filo queue empty".to_string(),
            ));
        }
    }

    if filo.is_empty() {
        Err(ChunkError::new(
            ErrorKind::NoLeftovers,
            "No leftovers".to_string(),
        ))
    } else {
        // Unfinished lines
        filo.reverse();
        Ok(filo.into_iter().fold(0, |acc, c| {
            (acc * 5) + get_score(get_closed(c).unwrap()).unwrap()
        }))
    }
}

impl From<String> for Data {
    fn from(string: String) -> Data {
        let mut points: Vec<u64> = Vec::new();
        for line in string.lines() {
            match parse(line) {
                Ok(score) => points.push(score),
                Err(err) => println!("{:?}: {}", err.kind, err.msg),
            }
        }
        points.sort_unstable();
        Data { points }
    }
}

fn main() {
    let filename = "resources/day_10.txt";

    let Data { points } = Data::from(read_to_string(filename).unwrap());

    println!("Middle score: {}", points[(points.len() / 2)]);
}
