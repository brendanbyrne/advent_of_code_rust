use std::collections::VecDeque;
use std::convert::From;
use std::fs::read_to_string;

struct Height {
    height: Vec<Vec<i32>>,
}

impl From<String> for Height {
    fn from(string: String) -> Height {
        let height: Vec<Vec<u32>> = {
            let mut height: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
            for line in string.lines() {
                height.push(
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                );
            }

            height
        };

        Height {
            height: height
                .into_iter()
                .map(|row| row.into_iter().map(|v| v as i32).collect())
                .collect(),
        }
    }
}

fn print<T: std::fmt::Display>(padded: &Vec<Vec<T>>) {
    for row in padded {
        for v in row {
            print!("{v}");
        }
        println!();
    }
}

fn find_basins(height: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let padded = {
        let mut padded = vec![vec![std::i32::MAX; height[0].len() + 2]; height.len() + 2];
        for i in 1..(height.len() + 1) {
            for j in 1..(height[1].len() + 1) {
                padded[i][j] = height[i - 1][j - 1];
            }
        }
        padded
    };

    {
        let mut basin_loc = Vec::<(usize, usize)>::new();

        for i in 1..(height.len() + 1) {
            for j in 1..(height[1].len() + 1) {
                let mut is_basin = true;
                for (oi, oj) in [
                    (-1_i32, -1_i32),
                    (-1_i32, 0_i32),
                    (-1_i32, 1_i32),
                    (0_i32, -1_i32),
                    (0_i32, 1_i32),
                    (1_i32, -1_i32),
                    (1_i32, 0_i32),
                    (1_i32, 1_i32),
                ] {
                    let ki = i as i32 + oi;
                    let kj = j as i32 + oj;
                    is_basin = is_basin && padded[i][j] < padded[ki as usize][kj as usize];
                }

                if is_basin {
                    basin_loc.push((i - 1, j - 1));
                }
            }
        }

        basin_loc
    }
}

#[derive(Clone)]
enum Status {
    Queued,
    Unknown,
    Visited,
    Wall,
}

fn find_valleys(height: &Vec<Vec<i32>>, basins: Vec<(usize, usize)>) -> usize {
    let padded = {
        let mut padded = vec![vec![9; height[0].len() + 2]; height.len() + 2];
        for i in 1..(height.len() + 1) {
            for j in 1..(height[1].len() + 1) {
                padded[i][j] = height[i - 1][j - 1];
            }
        }
        padded
    };

    let mut visited = vec![vec![Status::Wall; height[0].len() + 2]; height.len() + 2];
    for i in 1..(height.len() + 1) {
        for j in 1..(height[1].len() + 1) {
            visited[i][j] = {
                if padded[i][j] == 9 {
                    Status::Wall
                } else {
                    Status::Unknown
                }
            };
        }
    }

    let mut valleys: Vec<usize> = Vec::new();
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();

    for (start_i, start_j) in basins {
        let mut size = 0;
        to_visit.push_back((start_i + 1, start_j + 1));
        while !to_visit.is_empty() {
            let (i, j) = to_visit.pop_front().unwrap();

            visited[i][j] = Status::Visited;

            if padded[i][j] != 9 {
                size += 1;
            }

            for (oi, oj) in [
                (-1_i32, 0_i32),
                (1_i32, 0_i32),
                (0_i32, -1_i32),
                (0_i32, 1_i32),
            ] {
                let ki = (i as i32 + oi) as usize;
                let kj = (j as i32 + oj) as usize;

                match visited[ki][kj] {
                    Status::Unknown => {
                        visited[ki][kj] = Status::Queued;
                        to_visit.push_back((ki, kj));
                    }
                    _ => {}
                }
            }
        }
        valleys.push(size);
    }
    valleys.sort_unstable();
    valleys[valleys.len() - 3..].iter().product()
}

fn main() {
    let filename = "resources/day_9.txt";

    let Height { height } = Height::from(read_to_string(filename).unwrap());

    let basins = find_basins(&height);

    println!("Size: {}", find_valleys(&height, basins));
}
