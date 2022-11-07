use std::fs::File;
use std::io::prelude::*;

// Experiments in reading files
// fn get_depths_from_file(filename: &'static str) -> Result<Vec<i32>, &'static str> {
//     let lines = match std::fs::read_to_string(filename) {
//         Ok(v) => v.lines(),
//         _ => return Err("Failed to read file at {filename}"),
//     };

//     let mut depths = vec![];

//     for str_val in lines {
//         match str_val.parse::<i32>() {
//             Ok(v) => depths.push(v),
//             Err(e) => return Err("Failed to parse value {e}"),
//         }
//     }

//     Ok(depths)
// }

fn main() {
    // let filename = "/home/bbyrne/projects/advent_of_code_rust/day_1/resources/test_input.txt";
    let filename = "/home/bbyrne/projects/advent_of_code_rust/day_1/resources/input.txt";

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
