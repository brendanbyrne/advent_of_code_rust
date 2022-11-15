use ndarray::Array2;
use std::fs::read_to_string;

fn load_data<P: AsRef<std::path::Path>>(path: P) -> Option<Array2<u32>> {
    let data = match read_to_string(path) {
        Ok(string) => {
            let mut data: Vec<Vec<u32>> = Vec::new();
            for line in string.lines() {
                data.push(
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                );
            }
            data
        }
        _ => return None,
    };

    let mut energy = Array2::<u32>::zeros((data.len(), data[0].len()));

    for (i, row) in data.into_iter().enumerate() {
        for (j, v) in row.into_iter().enumerate() {
            energy[[i, j]] = v;
        }
    }

    Some(energy)
}

fn main() {
    // let filename = "resources/day_11.txt";
    let filename = "/home/bbyrne/tmp/test.txt";

    let mut energy = match load_data(filename) {
        Some(energy) => energy,
        _ => return,
    };

    {
        // Add 1
        energy += 1;

        // Find all fields > 9
        let mut flash_energy = Array2::<u32>::zeros(energy.shape());

        // propagate flashes

        // reset all fields > 9
    }
}
