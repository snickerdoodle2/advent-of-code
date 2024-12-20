use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    usize,
};

type Map = Vec<Vec<char>>;

#[allow(dead_code)]
fn print_map(map: &Map, path: Option<&Vec<(usize, usize, usize)>>) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Some(path) = path {
                if let Some((_, _, path)) = path.iter().find(|(px, py, _)| *px == x && *py == y) {
                    print!("{:0>2}", path);
                    continue;
                }
            }
            print!("{}{}", c, c);
        }
        println!();
    }
}

fn find_on_map(map: &Map, to_find: char) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == to_find {
                return (x, y);
            }
        }
    }

    unreachable!()
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn walk(
    map: &Map,
    start_x: usize,
    start_y: usize,
) -> Option<(Vec<(usize, usize, usize)>, HashMap<(usize, usize), usize>)> {
    let mut pq = BinaryHeap::new();
    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut end_pos = None;

    pq.push((Reverse(0), start_x, start_y));
    distance.insert((start_x, start_y), 0);

    while let Some((Reverse(dist), x, y)) = pq.pop() {
        for (dx, dy) in DIRS {
            let new_x: usize = (x as isize + dx) as usize;
            let new_y: usize = (y as isize + dy) as usize;
            if map[new_y][new_x] == '#' {
                continue;
            }

            if let Some(prev_dist) = distance.get(&(new_x, new_y)) {
                if *prev_dist <= dist + 1 {
                    continue;
                }
            }

            prev.insert((new_x, new_y), (x, y));
            distance.insert((new_x, new_y), dist + 1);
            if map[new_y][new_x] == 'E' {
                end_pos = Some((new_x, new_y));
                break;
            }
            pq.push((Reverse(dist + 1), new_x, new_y));
        }
    }

    if let Some((x, y)) = end_pos {
        let mut node_x = x;
        let mut node_y = y;
        let mut res = vec![];
        res.push((node_x, node_y, *distance.get(&(node_x, node_y)).unwrap()));
        while let Some((prev_x, prev_y)) = prev.get(&(node_x, node_y)) {
            node_x = *prev_x;
            node_y = *prev_y;
            res.push((node_x, node_y, *distance.get(&(node_x, node_y)).unwrap()));
        }

        res.reverse();

        return Some((res, distance));
    }

    None
}

#[inline]
fn in_bounds(x: isize, y: isize, width: isize, height: isize) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

pub fn process(input: &str) -> String {
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let (start_x, start_y) = find_on_map(&map, 'S');

    let Some((path, dist)) = walk(&map, start_x, start_y) else {
        return "Path not found".to_string();
    };

    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut res: Vec<usize> = vec![];

    for p in path {
        for (dx, dy) in DIRS {
            let x = p.0 as isize + dx;
            let y = p.1 as isize + dy;
            if !in_bounds(x, y, width, height) {
                continue;
            }
            if map[y as usize][x as usize] != '#' {
                continue;
            }
            let x = x + dx;
            let y = y + dy;
            if !in_bounds(x, y, width, height) {
                continue;
            }
            if let Some(dist) = dist.get(&(x as usize, y as usize)) {
                if *dist > p.2 {
                    res.push(*dist - p.2 - 2);
                    continue;
                }
            }
            let x = x + dx;
            let y = y + dy;
            if !in_bounds(x, y, width, height) {
                continue;
            }
            if let Some(dist) = dist.get(&(x as usize, y as usize)) {
                if *dist > p.2 {
                    res.push(*dist - p.2 - 3);
                }
            }
        }
    }

    res.into_iter().filter(|x| *x >= 100).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("0", process(input));
    }
}
