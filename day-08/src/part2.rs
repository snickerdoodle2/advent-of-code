use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    f32::EPSILON,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step_size(&self, other: &Self) -> (i32, i32) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        (x_diff, y_diff)
    }

    fn distance(&self, other: &Self) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

type Map = HashMap<char, Vec<Point>>;

fn parse(input: &str) -> Map {
    let mut map: Map = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, p)| {
            if p != '.' {
                map.entry(p).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        })
    });
    map
}

fn in_bounds(point: &Point, x_max: i32, y_max: i32) -> bool {
    point.x >= 0 && point.x < x_max && point.y >= 0 && point.y < y_max
}

pub fn process(input: &str) -> String {
    let map = parse(input);

    let mut res = HashSet::new();
    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;
    map.values().for_each(|v| {
        v.iter().permutations(2).for_each(|points| {
            let first = points[0];
            let second = points[1];
            let (x_step, y_step) = first.step_size(second);

            res.insert(*first);
            res.insert(*second);

            let mut p = *first;
            p.x -= x_step;
            p.y -= y_step;

            while in_bounds(&p, x_max, y_max) {
                res.insert(p);
                p.x -= x_step;
                p.y -= y_step;
            }
        })
    });

    res.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        "34"
    )]
    #[case(
        "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
        "9"
    )]
    fn test_part2(#[case] input: &str, #[case] output: &str) {
        assert_eq!(output, process(input));
    }
}
