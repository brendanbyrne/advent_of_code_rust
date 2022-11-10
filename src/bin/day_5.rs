use std::cmp::{max, min};
use std::fs::read_to_string;
use std::str::FromStr;

struct Diagram {
    height: Vec<Vec<usize>>,
}

#[derive(Default)]
struct Point {
    x: usize,
    y: usize,
}

struct Line {
    start: Point,
    end: Point,
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
        let mut line = Line {
            start: Default::default(),
            end: Default::default(),
        };

        let mut iter = s.split(" -> ");
        line.start = Point::from_str(iter.next().unwrap()).unwrap();
        line.end = Point::from_str(iter.next().unwrap()).unwrap();

        Ok(line)
    }
}

fn draw_line<I, J>(height: &mut Vec<Vec<usize>>, x_coords: I, y_coords: J)
where
    I: Iterator<Item = usize>,
    J: Iterator<Item = usize>,
{
    for (x, y) in x_coords.zip(y_coords) {
        // println!("Add height to ({x},{y})");
        height[y][x] += 1;
    }
}

fn align_range(i: usize, j: usize) -> std::ops::Range<usize> {
    min(i, j)..max(i, j) + 1 // +1 because it's inclusive to the range
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
            if line.start.x == line.end.x {
                // draw vertical line
                let x_coords = std::iter::repeat(line.start.x);
                let y_coords = align_range(line.start.y, line.end.y);

                draw_line(&mut height, x_coords, y_coords);
            } else if line.start.y == line.end.y {
                // draw horizontal line
                let x_coords = align_range(line.start.x, line.end.x);
                let y_coords = std::iter::repeat(line.start.y);

                draw_line(&mut height, x_coords, y_coords);
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
