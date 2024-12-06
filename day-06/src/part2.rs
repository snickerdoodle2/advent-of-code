use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn delta(&self) -> (i8, i8) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mark {
    Obstacle,
    Visited,
    Empty,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn step(self, dir: &Direction) -> Self {
        let (dx, dy) = dir.delta();
        let x = (self.x as i32) + (dx as i32);
        let y = (self.y as i32) + (dy as i32);

        Self { x, y }
    }
}

fn parse(input: &str) -> Vec<Vec<Mark>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Mark::Obstacle,
                    '^' => Mark::Visited,
                    '.' => Mark::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn find_starting_pos(map: &Vec<Vec<Mark>>) -> Coords {
    for (y, row) in map.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if *item == Mark::Visited {
                return Coords {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

    unreachable!()
}

fn print_map(map: &Vec<Vec<Mark>>) {
    for row in map {
        for x in row {
            let tmp = match x {
                Mark::Obstacle => '#',
                Mark::Visited => 'X',
                Mark::Empty => '.',
            };
            print!("{}", tmp);
        }
        println!();
    }
}

fn in_bounds(pos: &Coords, max_x: i32, max_y: i32) -> bool {
    pos.x >= 0 && pos.x <= max_x && pos.y >= 0 && pos.y <= max_y
}

fn is_loop(mut map: Vec<Vec<Mark>>, mut pos: Coords, obstacle: Coords) -> bool {
    map[obstacle.y as usize][obstacle.x as usize] = Mark::Obstacle;
    let max_y = map.len() as i32 - 1;
    let max_x = map[0].len() as i32 - 1;
    let mut direction = Direction::N;
    let mut visited = HashSet::new();

    while in_bounds(&pos, max_x, max_y) {
        let new_pos = pos.step(&direction);
        if !in_bounds(&new_pos, max_x, max_y) {
            break;
        };
        if map[new_pos.y as usize][new_pos.x as usize] == Mark::Obstacle {
            if visited.contains(&(pos, direction)) {
                return true;
            }
            visited.insert((pos, direction));
            direction = direction.next()
        } else {
            pos = new_pos;
        }
    }

    false
}

pub fn process(input: &str) -> String {
    let map = parse(input);
    let mut direction = Direction::N;
    let starting_pos = find_starting_pos(&map);
    let max_y = map.len() as i32 - 1;
    let max_x = map[0].len() as i32 - 1;

    let mut pos = starting_pos;
    let mut visited = HashSet::new();

    while in_bounds(&pos, max_x, max_y) {
        let new_pos = pos.step(&direction);
        if !in_bounds(&new_pos, max_x, max_y) {
            break;
        };
        if map[new_pos.y as usize][new_pos.x as usize] == Mark::Obstacle {
            direction = direction.next()
        } else {
            visited.insert(pos);
            pos = new_pos;
        }
    }
    visited.insert(pos);

    let res = visited
        .par_iter()
        .filter(|cand| is_loop(map.clone(), starting_pos, **cand))
        .count();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input));
    }
}
