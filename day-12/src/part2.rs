use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum EdgeType {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Edge(EdgeType, usize, usize);

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRS: [(isize, isize, EdgeType); 4] = [
    (1, 0, EdgeType::Right),
    (0, 1, EdgeType::Down),
    (-1, 0, EdgeType::Left),
    (0, -1, EdgeType::Up),
];

fn in_bounds(x: isize, y: isize, width: isize, height: isize) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn calculate(
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    map: &Map,
    visited: &mut Vec<Vec<bool>>,
) -> (u32, Option<HashSet<Edge>>) {
    if !in_bounds(x, y, width, height) || visited[y as usize][x as usize] {
        return (0, None);
    }

    let x = x as usize;
    let y = y as usize;

    visited[y][x] = true;

    let field = map[y][x];

    let mut area = 1;
    let mut edges = HashSet::new();

    for (dx, dy, edge_type) in DIRS {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if !in_bounds(new_x, new_y, width, height) || map[new_y as usize][new_x as usize] != field {
            edges.insert(Edge(edge_type, x, y));
        } else {
            let (new_area, new_edges) = calculate(new_x, new_y, width, height, map, visited);
            area += new_area;
            if let Some(new_edges) = new_edges {
                edges.extend(new_edges);
            }
        }
    }

    (area, Some(edges))
}

pub fn process(input: &str) -> String {
    let map = parse(input);
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    let mut visited = vec![vec![false; width as usize]; height as usize];

    let mut result = 0;

    for y in 0..height {
        for x in 0..width {
            let (area, edges) = calculate(x, y, width, height, &map, &mut visited);
            if let Some(edges) = edges {
                let mut edge_groups: HashMap<(EdgeType, usize), Vec<Edge>> = HashMap::new();

                edges.into_iter().for_each(|edge| {
                    let key = match edge.0 {
                        EdgeType::Up | EdgeType::Down => (edge.0, edge.2),
                        EdgeType::Left | EdgeType::Right => (edge.0, edge.1),
                    };

                    edge_groups.entry(key).or_default().push(edge);
                });

                let edge_count: usize = edge_groups
                    .into_iter()
                    .map(|((dir, _), mut edges)| {
                        edges.sort_by_key(match dir {
                            EdgeType::Up | EdgeType::Down => |x: &Edge| x.1,
                            EdgeType::Left | EdgeType::Right => |x: &Edge| x.2,
                        });

                        let mut edge_count = 1;
                        for (l, r) in edges.iter().tuple_windows() {
                            match dir {
                                EdgeType::Up | EdgeType::Down => {
                                    if r.1 - l.1 != 1 {
                                        edge_count += 1
                                    }
                                }
                                EdgeType::Left | EdgeType::Right => {
                                    if r.2 - l.2 != 1 {
                                        edge_count += 1
                                    }
                                }
                            }
                        }

                        edge_count
                    })
                    .sum();

                result += area as usize * edge_count;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "AAAA
    BBCD
    BBCC
    EEEC
    ",
        "80"
    )]
    #[case(
        "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        "236"
    )]
    fn test_part2(#[case] input: &str, #[case] output: String) {
        assert_eq!(output, process(input));
    }
}
