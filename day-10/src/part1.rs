use std::collections::HashSet;

type Map = Vec<Vec<i16>>;
type Cache = Vec<Vec<Option<HashSet<(usize, usize)>>>>;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vec(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i16)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn in_bounds(x: isize, y: isize, x_lim: usize, y_lim: usize) -> bool {
    x >= 0 && x < x_lim as isize && y >= 0 && y < y_lim as isize
}

const DIRECTION: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn calculate(
    map: &Map,
    cache: &mut Cache,
    x: isize,
    y: isize,
    x_lim: usize,
    y_lim: usize,
) -> Option<HashSet<(usize, usize)>> {
    if !in_bounds(x, y, x_lim, y_lim) {
        return None;
    }
    let x = x as usize;
    let y = y as usize;

    if map[y][x] == 9 {
        return Some(HashSet::from([(x, y)]));
    }
    if let Some(c) = &cache[y][x] {
        return Some(c.clone());
    }
    let num = map[y][x];

    let mut res = HashSet::new();

    for dir in DIRECTION {
        let (dx, dy) = dir.to_vec();
        let new_x = x as isize + dx as isize;
        let new_y = y as isize + dy as isize;
        if in_bounds(new_x, new_y, x_lim, y_lim) && map[new_y as usize][new_x as usize] - num == 1 {
            if let Some(s) = calculate(map, cache, new_x, new_y, x_lim, y_lim) {
                res.extend(s);
            }
        }
    }

    cache[y][x] = Some(res.clone());
    Some(res)
}

pub fn process(input: &str) -> String {
    let map = parse(input);
    let y_lim = input.lines().count();
    let x_lim = input.lines().next().unwrap().len();
    let mut cache: Cache = vec![vec![None; x_lim]; y_lim];

    let mut res = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, level) in row.iter().enumerate() {
            if *level == 0 {
                res += calculate(&map, &mut cache, x as isize, y as isize, x_lim, y_lim)
                    .map(|x| x.len())
                    .unwrap_or(0);
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input));
    }
}
