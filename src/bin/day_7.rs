use std::fs::read_to_string;

fn safe_diff(lhs: usize, rhs: usize) -> usize {
    std::cmp::max(lhs, rhs) - std::cmp::min(lhs, rhs)
}

fn main() {
    let filename = "resources/day_7.txt";

    let data = match read_to_string(filename) {
        Ok(string) => string
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
        _ => return,
    };

    let mut bins = vec![0 as u64; (data.iter().max().unwrap() + 1) as usize];
    let cost = {
        let mut v = vec![0 as usize; (data.iter().max().unwrap() + 1) as usize];
        for i in 1..v.len() {
            v[i] = i + v[i - 1];
        }
        v
    };

    for crab_loc in data.into_iter() {
        bins[crab_loc] += 1;
    }

    let mut min_fuel: u64 = std::u64::MAX;
    for rally_point in 0..bins.len() {
        let fuel = bins
            .iter()
            .enumerate()
            .map(|(i, b)| cost[safe_diff(i, rally_point)] as u64 * b)
            .sum();
        min_fuel = std::cmp::min(min_fuel, fuel);
    }

    println!("The minimum fuel cost is {}", min_fuel);
}
