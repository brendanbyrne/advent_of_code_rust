use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
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

    nodes[name_to_i["start"]].is_start = true;

    (nodes, name_to_i["start"], name_to_i["end"])
}

fn print(nodes: &Vec<Node>) {
    for n in nodes {
        println!("{}", n.name);
        println!("  is large: {}", n.is_large);
        println!("  idx: {}", n.idx);
        for i in &n.connections {
            println!("  - {}", nodes[*i].name);
        }
    }
}

#[derive(Clone)]
struct Path {
    path: Vec<usize>,
    visited: HashSet<usize>,
    done: bool,
    can_dup: bool,
}

impl Path {
    fn new() -> Path {
        Path {
            path: Vec::new(),
            visited: HashSet::new(),
            done: false,
            can_dup: true, // has opportunity to duplicate
        }
    }
}

fn pprint(p: &Path, nodes: &Vec<Node>) {
    let path_str = p
        .path
        .iter()
        .map(|i| nodes[*i].name.clone())
        .collect::<Vec<String>>()
        .join("->");
    let visited = p
        .visited
        .iter()
        .map(|i| nodes[*i].name.clone())
        .collect::<Vec<String>>()
        .join(" ");
    println!("Path: {path_str}\n    Visited: {visited}");
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        let lhs_name = self
            .path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let rhs_name = other
            .path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        lhs_name == rhs_name
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs_name = self
            .path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let rhs_name = other
            .path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        Some(lhs_name.cmp(&rhs_name))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs_name = self
            .path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let rhs_name = other
            .path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");
        lhs_name.cmp(&rhs_name)
    }
}

fn all_paths(nodes: &Vec<Node>, i_start: usize, i_end: usize) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    let mut keep_going = true;

    {
        let mut path = Path::new();
        path.path.push(i_start);
        paths.push(path);
    }

    while keep_going {
        keep_going = false;

        for i_p in 0..paths.len() {
            if paths[i_p].done {
                continue;
            }

            keep_going = true;

            // pprint(&paths[i_p], &nodes);

            let i_curr = paths[i_p].path.last().unwrap().clone();

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

            // let s = next_i_d
            //     .iter()
            //     .map(|(i, d)| format!("({}, {})", nodes[*i].name.clone(), d))
            //     .collect::<Vec<String>>()
            //     .join(", ");
            // println!("Pontential: {s}");

            // No where to go, branch is done
            if next_i_d.is_empty() {
                paths[i_p].done = true;
                continue;
            }

            // Add divergent paths
            if next_i_d.len() > 1 {
                for (i_next, can_dup) in &next_i_d[1..] {
                    paths.push(paths[i_p].clone());
                    paths.last_mut().unwrap().path.push(*i_next);
                    paths.last_mut().unwrap().can_dup = *can_dup;
                }
            }
            let (i_next, can_dup) = next_i_d[0];
            paths[i_p].path.push(i_next);
            paths[i_p].can_dup = can_dup;
        }
    }

    paths = paths
        .into_iter()
        .filter(|p| *p.path.last().unwrap() == i_end)
        .collect();
    paths.sort();
    paths
}

fn main() {
    let filename = "resources/day_12.txt";
    // let filename = "/home/bbyrne/tmp/test.txt";

    let (nodes, i_start, i_end) = load(filename);

    let paths = all_paths(&nodes, i_start, i_end);
    // for p in &paths {
    //     for i_n in p.path.clone() {
    //         print!("{} ", nodes[i_n].name);
    //     }
    //     print!("\n");
    // }
    println!("Number of paths {}", paths.len());
}
