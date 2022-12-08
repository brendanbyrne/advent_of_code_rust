use std::cmp::{max, min};
use std::convert::From;
use std::fmt;
use std::fs::read_to_string;
use std::str::Lines;

use lazy_static::lazy_static;
use ndarray::{s, Array2, Axis};
use regex::Regex;

enum Fold {
    X(usize),
    Y(usize),
}

struct Paper {
    dots: Array2<u8>,
}

impl From<Vec<(usize, usize)>> for Paper {
    fn from(pairs: Vec<(usize, usize)>) -> Paper {
        let shape = {
            let mut shape_x: usize = 0;
            let mut shape_y: usize = 0;
            for p in &pairs {
                shape_x = max(shape_x, p.0);
                shape_y = max(shape_y, p.1);
            }
            (shape_y + 1, shape_x + 1)
        };

        let mut dots: Array2<u8> = Array2::zeros(shape);
        for (x, y) in pairs {
            dots[[y, x]] = 1;
        }

        Paper { dots }
    }
}

impl Paper {
    fn new(x: usize, y: usize) -> Paper {
        Paper {
            dots: Array2::zeros((y, x)),
        }
    }

    fn fold_x(&mut self, x: usize) {}

    fn fold_y(&mut self, y: usize) {
	let mut folded = Paper::new(self.
        // println!(
        //     "{}",
        //     Paper {
        //         dots: self.dots.slice(s![0..y, ..]).to_owned()
        //     }
        // );

        // println!("___________");

        // let bottom = {
        //     let mut bottom = Paper {
        //         dots: self.dots.slice(s![y.., ..]).to_owned(),
        //     };
        //     bottom.dots.invert_axis(Axis(0));
        //     bottom
        // };

        // println!("{bottom}");

        // let top_slice = self.dots.slice(s![0..y, ..]);
        // self.dots.slice(s![y.., ..]).invert_axis(Axis(0));
        // // let bottom_slice =

        // let shape_y = min(top_slice.shape()[0], bottom_slice.shape()[0]);
        // let shape_x = top_slice.shape()[1];

        // let combined = {
        //     let mut combined = Paper::new(shape_x, shape_y);
        //     for ((i, j), v) in combined.dots.indexed_iter_mut() {
        //         v = top_slice[[i, j]] == 1 || bottom_slice[[i, j]] == 1;
        //     }
        // };
        // println!("Combined:\n{combined}",);
    }

    fn apply(&mut self, fold: Fold) {
        match fold {
            Fold::X(x) => self.fold_x(x),
            Fold::Y(y) => self.fold_y(y),
        }
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.dots.slice(s![..-1, ..]).rows() {
            for v in row {
                if *v == 1 {
                    let _ = write!(f, "#");
                } else {
                    let _ = write!(f, ".");
                }
            }
            let _ = write!(f, "\n");
        }

        for v in self.dots.row(self.dots.nrows() - 1) {
            if *v == 1 {
                let _ = write!(f, "#");
            } else {
                let _ = write!(f, ".");
            }
        }
        Ok(())
    }
}

struct Data {
    paper: Paper,
    folds: Vec<Fold>,
}

fn load_paper(lines: &mut Lines) -> Paper {
    let pairs = {
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut split = line.split(",");
            pairs.push((
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            ));
        }
        pairs
    };

    Paper::from(pairs)
}

fn load_folds(lines: &mut Lines) -> Vec<Fold> {
    lazy_static! {
        static ref FOLD_REGEX: Regex = Regex::new("fold along ([xy])=(\\d*)").unwrap();
    }

    let mut folds: Vec<Fold> = Vec::new();

    while let Some(line) = lines.next() {
        for caps in FOLD_REGEX.captures_iter(line) {
            let amount = &caps[2].parse::<usize>().unwrap();
            match &caps[1] {
                "x" => folds.push(Fold::X(*amount)),
                "y" => folds.push(Fold::Y(*amount)),
                _ => {}
            };
        }
    }

    folds
}

fn load<P: AsRef<std::path::Path>>(path: P) -> Data {
    let string = read_to_string(path).unwrap();
    let mut lines = string.lines();

    Data {
        paper: load_paper(&mut lines),
        folds: load_folds(&mut lines),
    }
}

fn main() {
    // let filename = "resources/day_13.txt";
    let filename = "/home/bbyrne/tmp/test.txt";

    let Data { mut paper, folds } = load(filename);

    println!("{paper}\n");

    for fold in folds {
        paper.apply(fold);
	break;
    }
}
