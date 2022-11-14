use std::fs::read_to_string;

fn update(fish: &mut [u64; 9]) {
    let num_to_spawn = fish[0];

    for i in 1..9 {
        fish[i - 1] = fish[i];
    }

    fish[6] += num_to_spawn;
    fish[8] = num_to_spawn;
}

fn main() {
    let filename = "resources/day_6.txt";

    let data: Vec<u64> = match read_to_string(filename) {
        Ok(string) => string
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
        _ => return,
    };

    let mut fish: [u64; 9] = [0; 9];
    for f in &data {
        fish[*f as usize] += 1;
    }

    for _ in 0..256 {
        update(&mut fish);
    }

    println!("number of fish: {}", fish.iter().sum::<u64>());
}
