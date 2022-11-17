use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

struct Node {
    name: String,
    idx: usize,
    connections: HashSet<usize>,
    is_large: bool,
    is_start: bool,
}

impl Node {
    fn new(name: String, idx: usize) -> Node {
        let is_large = name.chars().take(1).next().unwrap().is_uppercase();
        let is_start = name == "start";
        Node {
            name,
            idx,
            connections: HashSet::new(),
            is_large,
            is_start,
        }
    }
}

fn load<P: AsRef<std::path::Path>>(path: P) -> (Vec<Node>, usize, usize) {
    let mut nodes: Vec<Node> = Vec::new();

    let mut name_to_i: HashMap<String, usize> = HashMap::new();
    let mut i: usize = 0;
    let mut names: [String; 2] = [String::new(), String::new()];

    for line in read_to_string(path).unwrap().lines() {
        let mut iter = line.split("-");

        names[0] = iter.next().unwrap().to_string();
        names[1] = iter.next().unwrap().to_string();

        for name in &names {
            if !name_to_i.contains_key(name) {
                nodes.push(Node::new(name.clone(), i));
                name_to_i.insert(name.clone(), i);
                i += 1;
            }
        }

        nodes[*name_to_i.get(&names[0]).unwrap()]
            .connections
            .insert(*name_to_i.get(&names[1]).unwrap());
        nodes[*name_to_i.get(&names[1]).unwrap()]
            .connections
            .insert(*name_to_i.get(&names[0]).unwrap());
    }

    (nodes, name_to_i["start"], name_to_i["end"])
}

#[derive(Clone)]
struct Path {
    i_next: usize,
    visited: HashSet<usize>,
    done: bool,
    can_dup: bool,
}

impl Path {
    fn new(i_next: usize) -> Path {
        Path {
            i_next,
            visited: HashSet::new(),
            done: false,
            can_dup: true, // has opportunity to duplicate
        }
    }
}

fn all_paths(nodes: &Vec<Node>, i_start: usize, i_end: usize) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    let mut keep_going = true;

    paths.push(Path::new(i_start));

    while keep_going {
        keep_going = false;

        for i_p in 0..paths.len() {
            if paths[i_p].done {
                continue;
            }

            keep_going = true;

            let i_curr = paths[i_p].i_next;

            // Update state for current location
            if i_curr == i_end {
                paths[i_p].done = true;
                continue;
            } else if !nodes[i_curr].is_large {
                paths[i_p].visited.insert(i_curr);
            }

            // Generate potential next locations
            // Paired with what the value of can_dup will be after moving there
            let next_i_d = {
                let mut next_i_d: Vec<(usize, bool)> = Vec::new();

                for next_i in nodes[i_curr].connections.clone() {
                    if nodes[next_i].is_start {
                        continue;
                    } else if !paths[i_p].visited.contains(&next_i) {
                        next_i_d.push((next_i, paths[i_p].can_dup));
                    } else if paths[i_p].visited.contains(&next_i) && paths[i_p].can_dup {
                        // consumes the can_dup ability
                        next_i_d.push((next_i, false));
                    }
                }
                next_i_d
            };

            // No where to go, branch is done
            if next_i_d.is_empty() {
                paths[i_p].done = true;
                continue;
            }

            // Add divergent paths
            if next_i_d.len() > 1 {
                for (i_next, can_dup) in &next_i_d[1..] {
                    paths.push(paths[i_p].clone());
                    paths.last_mut().unwrap().i_next = *i_next;
                    paths.last_mut().unwrap().can_dup = *can_dup;
                }
            }
            let (i_next, can_dup) = next_i_d[0];
            paths[i_p].i_next = i_next;
            paths[i_p].can_dup = can_dup;
        }
    }

    paths = paths.into_iter().filter(|p| p.i_next == i_end).collect();
    paths
}

fn main() {
    let filename = "resources/day_12.txt";

    let (nodes, i_start, i_end) = load(filename);

    let paths = all_paths(&nodes, i_start, i_end);

    println!("Number of paths {}", paths.len());
}
