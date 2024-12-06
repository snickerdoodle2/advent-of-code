use std::collections::HashMap;

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

#[derive(Debug)]
enum Mark {
    Obstacle,
    Visited,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn step(self, dir: &Direction) -> Option<Self> {
        let (dx, dy) = dir.delta();
        let x = (self.x as i32) + (dx as i32);
        let y = (self.y as i32) + (dy as i32);
        if x < 0 || y < 0 {
            None
        } else {
            Some(Self {
                x: x as usize,
                y: y as usize,
            })
        }
    }
}

fn parse(input: &str) -> (HashMap<Coords, Mark>, Coords, (usize, usize)) {
    let mut map = HashMap::new();
    let mut pos = None;
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => {
                    map.insert(Coords { x, y }, Mark::Obstacle);
                }
                '^' => {
                    pos = Some(Coords { x, y });
                }
                _ => unreachable!(),
            }

            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    (map, pos.unwrap(), (max_x, max_y))
}

pub fn process(input: &str) -> String {
    let (mut map, mut pos, (max_x, max_y)) = parse(input);
    let mut visited = 0;
    let mut direction = Direction::N;

    loop {
        if pos.x > max_x || pos.y > max_y {
            break;
        }
        if map.insert(pos, Mark::Visited).is_none() {
            visited += 1;
        }
        let Some(new_pos) = pos.step(&direction) else {
            break;
        };
        if let Some(Mark::Obstacle) = map.get(&new_pos) {
            direction = direction.next();
            let Some(new_pos) = pos.step(&direction) else {
                break;
            };
            pos = new_pos
        } else {
            pos = new_pos;
        }
    }

    visited.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!("41", process(input));
    }
}
