use std::fs::read_to_string;
use std::str::FromStr;

struct Contents {
    numbers: Vec<i32>,
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
            boards: Vec::<[[i32; 5]; 5]>::new(),
        };

        while let Some(mut line) = iter.next() {
            if line.is_empty() {
                continue;
            }

            let mut board = [[0; 5]; 5];

            for (j, v_str) in line.split_whitespace().enumerate() {
                board[0][j] = v_str.parse::<i32>().unwrap();
            }

            // Start at 1 because first .unwrap() is performed by the while loop
            for i in 1..5 {
                line = iter.next().unwrap();
                for (j, v_str) in line.split_whitespace().enumerate() {
                    board[i][j] = v_str.parse::<i32>().unwrap();
                }
            }

            contents.boards.push(board);
        }
        Ok(contents)
    }
}

fn main() {
    let filename = "/home/bbyrne/projects/advent_of_code_rust/resources/day_4.txt";

    let Contents { numbers, boards } = match read_to_string(filename) {
        Ok(string) => Contents::from_str(&string).unwrap(),
        _ => return,
    };
}
