use std::fs::read_to_string;

#[derive(Debug)]
enum ErrorKind {
    UnknownCharacter,
    Unfinished,
    EmptyFilo,
}

#[derive(Debug)]
struct ChunkError {
    kind: ErrorKind,
    msg: String,
}

impl ChunkError {
    fn new(kind: ErrorKind, msg: &str) -> ChunkError {
        ChunkError {
            kind,
            msg: msg.to_string(),
        }
    }
}

fn get_score(closed: char) -> Result<u32, ChunkError> {
    match closed {
        ')' => Ok(3),
        ']' => Ok(57),
        '}' => Ok(1197),
        '>' => Ok(25137),
        _ => Err(ChunkError::new(
            ErrorKind::UnknownCharacter,
            "Attempted to score an ungraded character: {closed}",
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
            "No open character for {closed}",
        )),
    }
}

fn is_open(c: char) -> Result<bool, ChunkError> {
    match c {
        '(' | '[' | '{' | '<' => Ok(true),
        ')' | ']' | '}' | '>' => Ok(false),
        _ => Err(ChunkError::new(
            ErrorKind::UnknownCharacter,
            "Not an open or closed state for {c}",
        )),
    }
}

struct Data {
    points: Vec<u32>,
}

fn parse(line: &str) -> Result<u32, ChunkError> {
    let mut filo: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_open(c)? {
            filo.push(c);
        } else {
            if let Some(back) = filo.last() {
                if *back == get_open(c)? {
                    filo.pop();
                } else {
                    return Ok(get_score(c)?);
                }
            } else {
                // Encounted empty queue unexpectedly
                return Err(ChunkError::new(ErrorKind::EmptyFilo, "Filo queue empty"));
            }
        }
    }

    if filo.is_empty() {
        Ok(0)
    } else {
        // Unfinished lines
        Err(ChunkError::new(
            ErrorKind::Unfinished,
            "Characters still left",
        ))
    }
}

impl From<String> for Data {
    fn from(string: String) -> Data {
        let mut points: Vec<u32> = Vec::new();
        for line in string.lines() {
            match parse(line) {
                Ok(score) => points.push(score),
                Err(err) => println!("{:?}: {}", err.kind, err.msg),
            }
        }
        Data { points }
    }
}

fn main() {
    let filename = "resources/day_10.txt";

    let Data { points } = Data::from(read_to_string(filename).unwrap());

    println!("Points: {}", points.into_iter().sum::<u32>());
}
