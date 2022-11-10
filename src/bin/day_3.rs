use itertools::izip;
use std::cmp::{max, min};
use std::fs::File;
use std::io::prelude::*;

fn binary_to_decimal(bits: &Vec<i32>) -> i32 {
    let mut num = 0;
    for (i, b) in bits.iter().rev().enumerate() {
        num += b * 2_i32.pow(i.try_into().unwrap());
    }

    num
}

fn find_oxygen(mut report: Vec<String>, bit_depth: usize) -> i32 {
    let mut cmp: char;
    let mut bit = 0;
    while report.len() > 1 && bit < bit_depth {
        let mut count = 0;
        cmp = '0';

        for line in &report {
            if line.chars().nth(bit) == Some('1') {
                count += 1;
            }
        }

        if max(count, report.len() - count) == count {
            cmp = '1';
        }

        report = report
            .into_iter()
            .filter(|s| s.chars().nth(bit) == Some(cmp))
            .collect();

        bit += 1
    }

    let bits = report[0].chars().map(|c| c as i32 - '0' as i32).collect();

    binary_to_decimal(&bits)
}

fn find_co2(mut report: Vec<String>, bit_depth: usize) -> i32 {
    let mut cmp: char;
    let mut bit = 0;
    while report.len() > 1 && bit < bit_depth {
        let mut count = 0;
        cmp = '1';

        for line in &report {
            if line.chars().nth(bit) == Some('1') {
                count += 1;
            }
        }

        if max(count, report.len() - count) == count {
            cmp = '0';
        }

        report = report
            .into_iter()
            .filter(|s| s.chars().nth(bit) == Some(cmp))
            .collect();

        bit += 1
    }

    let bits = report[0].chars().map(|c| c as i32 - '0' as i32).collect();

    binary_to_decimal(&bits)
}

fn find_gamma_and_epsilon(report: &Vec<String>, bit_depth: usize) -> (i32, i32) {
    let mut counts = vec![0; bit_depth];

    let size = report.len();

    for r in report {
        for (v, count) in izip!(r.chars(), counts.iter_mut()) {
            if v == '1' {
                *count += 1;
            }
        }
    }

    let mut gamma_bits = vec![0; bit_depth];
    let mut epsilon_bits = vec![0; bit_depth];

    for (c, g, e) in izip!(counts, gamma_bits.iter_mut(), epsilon_bits.iter_mut()) {
        if max(c, size - c) == c {
            *g = 1;
        } else {
            *g = 0;
        }
        if min(c, size - c) == c {
            *e = 1;
        } else {
            *e = 0;
        }
    }

    (
        binary_to_decimal(&gamma_bits),
        binary_to_decimal(&epsilon_bits),
    )
}

fn main() {
    let filename = "resources/day_3.txt";

    let mut f = File::open(filename).expect("File not found: {filename}");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading {filename}");

    let report: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let bit_depth = report[0].len();

    let oxygen = find_oxygen(report.clone(), bit_depth);
    println!("Oxygen: {}", oxygen);

    let co2 = find_co2(report.clone(), bit_depth);
    println!("CO2: {}", co2);

    let (gamma, epsilon) = find_gamma_and_epsilon(&report, bit_depth);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("gamma * epsilon: {}", gamma * epsilon);
    println!("oxy * CO2: {}", oxygen * co2);
}
