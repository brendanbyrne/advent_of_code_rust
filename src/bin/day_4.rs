use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

struct Index {
    b: usize, // board number
    i: usize, // row
    j: usize, // column
}

struct Contents {
    numbers: Vec<i32>,
    locations: HashMap<i32, Vec<Index>>,
    boards: Vec<[[i32; 5]; 5]>,
}

#[derive(Debug)]
struct ParseContentsError {}

impl FromStr for Contents {
    type Err = ParseContentsError;

    fn from_str(s: &str) -> Result<Contents, Self::Err> {
        let mut iter = s.lines();
        let mut contents = Self {
            numbers: iter
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
            locations: HashMap::new(),
            boards: Vec::<[[i32; 5]; 5]>::new(),
        };

        let mut b = 0;
        while let Some(mut line) = iter.next() {
            if line.is_empty() {
                continue;
            }

            let mut board = [[0; 5]; 5];

            for (j, v_str) in line.split_whitespace().enumerate() {
                let v = v_str.parse::<i32>().unwrap();
                contents
                    .locations
                    .entry(v)
                    .or_insert(vec![])
                    .push(Index { b: b, i: 0, j: j });
                board[0][j] = v;
            }

            // Starts at 1 because first .unwrap() is performed by the while loop
            for i in 1..5 {
                line = iter.next().unwrap();
                for (j, v_str) in line.split_whitespace().enumerate() {
                    let v = v_str.parse::<i32>().unwrap();
                    contents
                        .locations
                        .entry(v)
                        .or_insert(vec![])
                        .push(Index { b: b, i: i, j: j });
                    board[i][j] = v;
                }
            }

            contents.boards.push(board);

            b += 1;
        }

        Ok(contents)
    }
}

fn check_move(state: &[[bool; 5]; 5], i_pos: &usize, j_pos: &usize) -> bool {
    if state[*i_pos].iter().all(|v| *v) {
        return true;
    }

    for i in 0..5 {
        if !state[i][*j_pos] {
            return false;
        }
    }

    return true;
}

fn score_board(board: &[[i32; 5]; 5], state: &[[bool; 5]; 5]) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !state[i][j] {
                sum += board[i][j];
            }
        }
    }
    sum
}

fn main() {
    let filename = "/home/bbyrne/projects/advent_of_code_rust/resources/day_4.txt";

    let Contents {
        numbers,
        locations,
        boards,
    } = match read_to_string(filename) {
        Ok(string) => Contents::from_str(&string).unwrap(),
        _ => return,
    };

    let mut states = vec![[[false; 5]; 5]; boards.len()];

    let mut boards_won = vec![false; boards.len()];

    'outer: for n in numbers {
        for Index { b, i, j } in locations.get(&n).unwrap() {
            states[*b][*i][*j] = true;

            if check_move(&states[*b], i, j) {
                boards_won[*b] = true;

                if boards_won.iter().all(|w| *w) {
                    let score = score_board(&boards[*b], &states[*b]) * n;
                    println!("Won with score: {score}");
                    break 'outer;
                }
            }
        }
    }
}
