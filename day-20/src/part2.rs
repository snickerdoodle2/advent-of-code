use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

use nom::sequence::preceded;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

fn count_cheats(
    start_x: isize,
    start_y: isize,
    og_distances: &HashMap<(usize, usize), usize>,
    width: isize,
    height: isize,
    min_distance: i32,
) -> usize {
    const DIST: isize = 20;

    let mut candidates = HashSet::new();
    for dx in 0..=DIST {
        let dy = DIST - dx;
        for new_x in (start_x - dx).max(0)..=(start_x + dx).min(width - 1) {
            for new_y in (start_y - dy).max(0)..=(start_y + dy).min(height - 1) {
                if !(new_x == start_x && new_y == start_y) {
                    candidates.insert((
                        new_x,
                        new_y,
                        (new_x - start_x).abs() + (new_y - start_y).abs(),
                    ));
                }
            }
        }
    }

    let start_dist = og_distances
        .get(&(start_x as usize, start_y as usize))
        .unwrap();

    candidates
        .into_iter()
        .filter_map(|(x, y, d)| {
            let dist = *og_distances.get(&(x as usize, y as usize))?;
            // dbg!((dist as i32 - *start_dist as i32, min_distance + d as i32));
            if dist as i32 - *start_dist as i32 >= min_distance + d as i32 {
                Some(())
            } else {
                None
            }
        })
        .count()
}

pub fn process(input: &str, min_d: i32) -> String {
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let (start_x, start_y) = find_on_map(&map, 'S');

    let Some((path, dist)) = walk(&map, start_x, start_y) else {
        return "Path not found".to_string();
    };

    let height = map.len() as isize;
    let width = map[0].len() as isize;

    path.par_iter()
        .map(|(x, y, _)| count_cheats(*x as isize, *y as isize, &dist, width, height, min_d))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!("41", process(input, 70));
    }
}
