use std::cmp::max;
use std::fs::read_to_string;
use std::iter::IntoIterator;
use std::str::FromStr;

struct Diagram {
    height: Vec<Vec<usize>>,
}

struct Point {
    x: usize,
    y: usize,
}

struct Line {
    start: Point,
    end: Point,
}

fn increment(curr: &mut usize, target: &usize) -> bool {
    if *curr > *target {
        *curr -= 1;
        return true;
    } else if *curr < *target {
        *curr += 1;
        return true;
    }

    false
}

impl IntoIterator for Line {
    type Item = (usize, usize);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut points = Vec::<(usize, usize)>::new();

        let mut x = self.start.x;
        let mut y = self.start.y;

        let mut x_can_move = true;
        let mut y_can_move = true;
        while x_can_move || y_can_move {
            points.push((x, y));

            x_can_move = increment(&mut x, &self.end.x);
            y_can_move = increment(&mut y, &self.end.y);
        }

        points.into_iter()
    }
}

impl FromStr for Point {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let mut point = Point { x: 0, y: 0 };

        let mut iter = s.split(',');
        point.x = iter.next().unwrap().parse::<usize>().unwrap();
        point.y = iter.next().unwrap().parse::<usize>().unwrap();

        Ok(point)
    }
}

impl FromStr for Line {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Line, Self::Err> {
        let mut iter = s.split(" -> ");

        let line = Line {
            start: Point::from_str(iter.next().unwrap()).unwrap(),
            end: Point::from_str(iter.next().unwrap()).unwrap(),
        };

        Ok(line)
    }
}

impl FromStr for Diagram {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Diagram, Self::Err> {
        let mut max_point = Point { x: 0, y: 0 };
        let lines: Vec<Line> = s
            .lines()
            .map(|l| {
                let line = Line::from_str(l).unwrap();
                max_point.x = max(max_point.x, max(line.start.x, line.end.x));
                max_point.y = max(max_point.y, max(line.start.y, line.end.y));
                line
            })
            .collect();

        // height[y][x]
        // + 1 because coordinates come zero-indexed
        let mut height = vec![vec![0; max_point.x + 1]; max_point.y + 1];

        for line in lines {
            for (x, y) in line {
                height[y][x] += 1;
            }
        }

        Ok(Diagram { height })
    }
}

fn main() {
    let filename = "resources/day_5.txt";

    let Diagram { height } = match read_to_string(filename) {
        Ok(string) => Diagram::from_str(&string).unwrap(),
        _ => return,
    };

    let num_overlaps = height.into_iter().flatten().filter(|h| h >= &2).count();
    println!("number of overlaps: {num_overlaps}");
}
