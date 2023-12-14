use std::collections::{BTreeMap, HashMap};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Round,
    Cube,
}

impl Rock {
    fn new(c: char) -> Option<Self> {
        match c {
            'O' => Some(Rock::Round),
            '#' => Some(Rock::Cube),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

// BTreeMap cuz it sorts by key
fn parse_input(input: &str) -> (BTreeMap<(usize, usize), Rock>, usize, usize) {
    let tmp = input.lines().collect::<Vec<_>>();
    let height = tmp.len();
    let width = tmp.first().unwrap().chars().collect::<Vec<_>>().len();
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if let Some(rock) = Rock::new(c) {
                            Some(((x, y), rock))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        height,
        width,
    )
}

pub fn process_1(input: &str) -> String {
    let (mut rocks, height, _) = parse_input(input);

    rocks
        .clone()
        .into_iter()
        .for_each(|((x, y), rock)| match rock {
            Rock::Cube => (),
            Rock::Round => {
                if y > 0 {
                    let mut new_y = None;
                    for yp in (0..y).rev() {
                        match rocks.get(&(x, yp)) {
                            None => (),
                            Some(_) => {
                                new_y = Some(yp + 1);
                                break;
                            }
                        }

                        new_y = Some(0);
                    }
                    if let Some(new_y) = new_y {
                        rocks.remove(&(x, y));
                        rocks.insert((x, new_y), rock);
                    }
                }
            }
        });

    rocks
        .iter()
        .filter_map(|((_, y), rock)| match rock {
            Rock::Cube => None,
            Rock::Round => Some(height - y),
        })
        .sum::<usize>()
        .to_string()
}

fn grid_key(rocks: &BTreeMap<(usize, usize), Rock>, height: &usize, width: &usize) -> String {
    let mut out = "".to_string();
    for y in 0..*height {
        for x in 0..*width {
            out += match rocks.get(&(x, y)) {
                Some(rock) => match rock {
                    Rock::Round => "O",
                    Rock::Cube => "#",
                },
                None => ".",
            }
        }
        out += "\n";
    }
    out
}

pub fn process_2(input: &str) -> String {
    use Direction::*;
    let (mut rocks, height, width) = parse_input(input);
    let mut cache: HashMap<String, usize> = HashMap::new();
    'outer: for i in 1usize..=1_000_000_000 {
        for direction in vec![North, West, South, East] {
            if direction == South || direction == East {
                rocks
                    .clone()
                    .into_iter()
                    .rev()
                    .for_each(|((x, y), rock)| match rock {
                        Rock::Cube => (),
                        Rock::Round => {
                            let mut new_pos = None;
                            match direction {
                                South => {
                                    for yp in (y + 1)..height {
                                        match rocks.get(&(x, yp)) {
                                            None => (),
                                            Some(_) => {
                                                new_pos = Some((x, yp - 1));
                                                break;
                                            }
                                        }

                                        new_pos = Some((x, height - 1));
                                    }
                                }
                                East => {
                                    for xp in (x + 1)..width {
                                        match rocks.get(&(xp, y)) {
                                            None => (),
                                            Some(_) => {
                                                new_pos = Some((xp - 1, y));
                                                break;
                                            }
                                        }

                                        new_pos = Some((width - 1, y));
                                    }
                                }
                                _ => panic!(),
                            }
                            if let Some(new_pos) = new_pos {
                                rocks.remove(&(x, y));
                                rocks.insert(new_pos, rock);
                            }
                        }
                    });
            } else {
                rocks
                    .clone()
                    .into_iter()
                    .for_each(|((x, y), rock)| match rock {
                        Rock::Cube => (),
                        Rock::Round => {
                            let mut new_pos = None;
                            match direction {
                                North => {
                                    for yp in (0..y).rev() {
                                        match rocks.get(&(x, yp)) {
                                            None => (),
                                            Some(_) => {
                                                new_pos = Some((x, yp + 1));
                                                break;
                                            }
                                        }

                                        new_pos = Some((x, 0));
                                    }
                                }
                                West => {
                                    for xp in (0..x).rev() {
                                        match rocks.get(&(xp, y)) {
                                            None => (),
                                            Some(_) => {
                                                new_pos = Some((xp + 1, y));
                                                break;
                                            }
                                        }

                                        new_pos = Some((0, y));
                                    }
                                }
                                _ => panic!(),
                            }
                            if let Some(new_pos) = new_pos {
                                rocks.remove(&(x, y));
                                rocks.insert(new_pos, rock);
                            }
                        }
                    });
            }
        }

        let key = grid_key(&rocks, &height, &width);

        if let Some(cycle_start) = cache.get(&key) {
            let cycle_size = i - cycle_start;
            let target_cycle = cycle_start + (1_000_000_000 - cycle_start) % cycle_size;

            for (k, v) in &cache {
                if *v == target_cycle {
                    let (rocks_tmp, _, _) = parse_input(k);
                    rocks = rocks_tmp;
                    break 'outer;
                }
            }
        } else {
            cache.insert(key, i);
        }
    }

    rocks
        .iter()
        .filter_map(|((_, y), rock)| match rock {
            Rock::Cube => None,
            Rock::Round => Some(height - y),
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let res = process_1(input);
        assert_eq!("136", res);
    }

    #[test]
    fn test_process_2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let res = process_2(input);
        assert_eq!("64", res);
    }
}
