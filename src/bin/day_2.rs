use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    value: i32,
}

#[derive(Debug)]
pub struct ParseCommandError {}

// Attempt to use traits so `parse::<Direction>(string)` works
impl FromStr for Direction {
    type Err = ParseCommandError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(ParseCommandError {}),
        }
    }
}

fn main() {
    let filename = "resources/day_2.txt";

    let mut f = File::open(filename).expect("File not found: {filename}");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading {filename}");

    let mut commands = Vec::<Command>::new();
    for c in contents.lines() {
        let mut it = c.split(' ');
        commands.push(Command {
            direction: it.next().unwrap().parse::<Direction>().unwrap(),
            value: it.next().unwrap().parse::<i32>().unwrap(),
        });
    }

    let mut aim = 0;
    let mut horiz_pos = 0;
    let mut depth = 0;
    for c in commands {
        match c.direction {
            Direction::Up => aim -= c.value,
            Direction::Down => aim += c.value,
            Direction::Forward => {
                horiz_pos += c.value;
                depth += aim * c.value;
            }
        }
    }

    println!("{}", horiz_pos * depth);
}
