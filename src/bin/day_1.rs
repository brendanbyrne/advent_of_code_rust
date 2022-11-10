use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "resources/day_1.txt";

    let mut f = File::open(filename).expect("File not found: {filename}");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading {filename}");

    let mut depths = Vec::new();

    for c in contents.lines() {
        match c.parse::<i32>() {
            Ok(v) => depths.push(v),
            _ => return,
        }
    }

    let mut windows = depths.windows(3);

    let mut prev: i32 = windows.next().unwrap().iter().sum();
    let mut curr;
    let mut greater_count = 0;

    for w in windows {
        curr = w.iter().sum();
        if curr > prev {
            greater_count += 1;
        }
        prev = curr;
    }

    println!("Increased {} times", greater_count);
}
