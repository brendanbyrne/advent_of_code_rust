use std::fs::read_to_string;

#[derive(Clone)]
struct Jellyfish {
    energy: u32,
    should_flash: bool,
    allow_adds: bool,
}

impl Jellyfish {
    fn new(energy: u32) -> Jellyfish {
        let should_flash = energy > 9;
        Jellyfish {
            energy,
            should_flash,
            allow_adds: !should_flash,
        }
    }

    fn add(&mut self, e: u32) {
        if self.allow_adds {
            self.energy += e;

            if self.energy > 9 {
                self.should_flash = true;
                self.allow_adds = false;
            }
        }
    }
}

type Jellyfish2D = Vec<Vec<Jellyfish>>;

fn load_data<P: AsRef<std::path::Path>>(path: P) -> Option<Jellyfish2D> {
    match read_to_string(path) {
        Ok(string) => {
            let mut data: Jellyfish2D = Vec::new();
            for line in string.lines() {
                data.push(
                    line.chars()
                        .map(|c| Jellyfish::new(c.to_digit(10).unwrap()))
                        .collect::<Vec<Jellyfish>>(),
                );
            }
            return Some(data);
        }
        _ => return None,
    };
}

fn lower(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        x - 1
    }
}

fn upper(x: usize, max: usize) -> usize {
    if x + 2 > max {
        max
    } else {
        x + 2
    }
}

fn update_surrounding(jfish: &mut Jellyfish2D, i: usize, j: usize) {
    for i_a in lower(i)..upper(i, jfish.len()) {
        for j_a in lower(j)..upper(j, jfish[i_a].len()) {
            jfish[i_a][j_a].add(1);
        }
    }
}

fn main() {
    let filename = "resources/day_11.txt";

    let mut jfish = match load_data(filename) {
        Some(jfish) => jfish,
        _ => return,
    };

    let mut step = 0;
    let mut seen_synced_flash = false;
    while !seen_synced_flash {
        step += 1;

        for i in 0..jfish.len() {
            for j in 0..jfish[i].len() {
                jfish[i][j].add(1);
            }
        }

        let mut still_flashing = true;
        while still_flashing {
            still_flashing = false;
            for i in 0..jfish.len() {
                for j in 0..jfish[i].len() {
                    if jfish[i][j].should_flash {
                        still_flashing = true;

                        update_surrounding(&mut jfish, i, j);

                        jfish[i][j].energy = 0;
                        jfish[i][j].should_flash = false;
                    }
                }
            }
        }

        seen_synced_flash = true;
        for i in 0..jfish.len() {
            for j in 0..jfish[i].len() {
                seen_synced_flash =
                    seen_synced_flash && (!jfish[i][j].should_flash && !jfish[i][j].allow_adds);
            }
        }

        for i in 0..jfish.len() {
            for j in 0..jfish[i].len() {
                jfish[i][j].allow_adds = true;
            }
        }
    }

    println!("Synced flash at step: {step}");
}
