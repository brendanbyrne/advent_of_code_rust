use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::fs::read_to_string;

type Chars = Vec<char>;

struct Observation {
    patterns: Vec<Chars>,
    outputs: Vec<Chars>,
}

struct Data {
    observations: Vec<Observation>,
}

impl From<&str> for Observation {
    fn from(s: &str) -> Self {
        let mut splitter = s.split(" | ");
        let patterns: Vec<Chars> = splitter
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.chars().collect::<Chars>())
            .collect();
        let outputs: Vec<Chars> = splitter
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.chars().collect::<Chars>())
            .collect();
        Observation { patterns, outputs }
    }
}

impl From<&str> for Data {
    fn from(s: &str) -> Self {
        let observations: Vec<Observation> = s
            .lines()
            .map(|v| Observation::from(v))
            .collect::<Vec<Observation>>();
        Data { observations }
    }
}

fn decode(patterns: &Vec<Chars>) -> HashMap<char, char> {
    let len_to_num: HashMap<usize, i32> = HashMap::from([(2, 1), (3, 7), (4, 4), (7, 8)]);

    let mut seg_guess: HashMap<char, HashSet<char>> = {
        let seg_iter = 'a'..='g';
        let seg_set: HashSet<char> = seg_iter.clone().collect();
        seg_iter.zip(std::iter::repeat(seg_set)).collect()
    };

    //   a
    //  b c
    //   d
    //  e f
    //   g
    let true_num_to_seg: HashMap<i32, HashSet<char>> = HashMap::from([
        (0, HashSet::from(['a', 'b', 'c', 'e', 'f', 'g'])),
        (1, HashSet::from(['c', 'f'])),
        (2, HashSet::from(['a', 'c', 'd', 'e', 'g'])),
        (3, HashSet::from(['a', 'c', 'd', 'f', 'g'])),
        (4, HashSet::from(['b', 'c', 'd', 'f'])),
        (5, HashSet::from(['a', 'b', 'd', 'f', 'g'])),
        (6, HashSet::from(['a', 'b', 'd', 'e', 'f', 'g'])),
        (7, HashSet::from(['a', 'c', 'f'])),
        (8, HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
        (9, HashSet::from(['a', 'b', 'c', 'd', 'f'])),
    ]);

    let mut num_to_pattern: HashMap<i32, &Chars> = HashMap::new();

    // Find the easy ones first
    for p in patterns {
        if let Some(num) = len_to_num.get(&p.len()) {
            num_to_pattern.insert(*num, p);
            for s in p {
                seg_guess.insert(*s, true_num_to_seg[num].clone());
            }
        }
    }

    {
        let one_segs: HashSet<char> = num_to_pattern[&1].clone().into_iter().collect();

        // Find 3
        for p in patterns.iter().filter(|p| p.len() == 5) {
            let p_segs: HashSet<char> = p.clone().into_iter().collect();
            if one_segs.intersection(&p_segs).count() == one_segs.len() {
                num_to_pattern.insert(3, p);
                println!("3 found");
            }
        }

        // Find 0 and 6
        for p in patterns.iter().filter(|p| p.len() == 6) {
            let p_segs: HashSet<char> = p.clone().into_iter().collect();
            // Find 0
            // let intersect = one_segs.intersection(&p_segs);
            // let size = intersect.count();
            let one_in_p: HashSet<char> = one_segs.intersection(&p_segs).collect();

            if one_in_p.len() == 2 {
                num_to_pattern.insert(0, p);
                println!("0 found");
            } else if one_in_p.len() == 1 {
                num_to_pattern.insert(6, p);
                println!("6 found");

                // let intersection = intersect.clone()..collect();
                seg_guess.insert(
                    'f',
                    one_in_p
                        .intersection(seg_guess.get(&'f').unwrap())
                        .collect(),
                    // seg_guess
                    //     .get(&'f')
                    //     .unwrap()
                    //     .intersection(&intersection)
                    //     .collect(),
                );
                seg_guess.insert(
                    'c',
                    seg_guess.get(&'c').unwrap().difference(one_in_p).collect(),
                );
            }
        }
    }

    { // Map ones segements
    }

    // mapping from scrambled wiring to expected wiring
    let seg_to_seg = HashMap::new();
    seg_to_seg
}

fn solve(observation: &Observation) -> u32 {
    let segment_map = decode(&observation.patterns);
    0
}

fn main() {
    // let filename = "resources/day_8.txt";
    let filename = "/home/bbyrne/tmp/test.txt";

    let s = read_to_string(filename).unwrap();
    let Data { observations } = Data::from(s.as_str());

    let total: u32 = observations.iter().map(|o| solve(o)).sum();
    println!("Sum: {}", total);
}
