use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_vec(&self) -> (i8, i8) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn next_counterclockwise(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn next_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

type Map = Vec<Vec<char>>;

struct QueueItem(u32, usize, usize, Direction, Vec<(usize, usize)>);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for QueueItem {}

fn walk(map: &Map, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(QueueItem(0, x, y, Direction::East, vec![(x, y)])));
    let mut cache = HashMap::new();
    let mut best_path = None;
    let mut res = HashSet::new();

    while let Some(Reverse(QueueItem(cost, x, y, dir, mut path))) = heap.pop() {
        let item = map[y][x];
        if item == 'E' {
            if let Some(cur_best) = best_path {
                if cost == cur_best {
                    res.extend(&path);
                }
            } else {
                res.extend(&path);
                best_path = Some(cost);
            }
            continue;
        }
        if item == '#' {
            continue;
        }

        if let Some(cached) = cache.get(&(x, y, dir)) {
            if *cached < cost {
                continue;
            }
        }

        cache.insert((x, y, dir), cost);

        heap.push(Reverse(QueueItem(
            cost + 1000,
            x,
            y,
            dir.next_clockwise(),
            path.clone(),
        )));

        heap.push(Reverse(QueueItem(
            cost + 1000,
            x,
            y,
            dir.next_counterclockwise(),
            path.clone(),
        )));

        let (dx, dy) = dir.to_vec();
        let x = (x as isize + dx as isize) as usize;
        let y = (y as isize + dy as isize) as usize;
        path.push((x, y));
        heap.push(Reverse(QueueItem(cost + 1, x, y, dir, path)));
    }

    res
}

fn find(map: &Map, target: char) -> Option<(usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == target {
                return Some((x, y));
            }
        }
    }

    None
}

pub fn process(input: &str) -> String {
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let (x, y) = find(&map, 'S').unwrap();

    walk(&map, x, y).len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "45"
    )]
    #[case(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        "64"
    )]
    fn test_part2(#[case] input: &str, #[case] expected: String) {
        assert_eq!(expected, process(input));
    }
}
