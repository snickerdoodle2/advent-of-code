use std::collections::HashMap;

#[derive(Debug)]
enum Splitter {
    Vertical,   // |
    Horizontal, // -
}

impl Splitter {
    fn get_next_dirs(&self, dir: &Direction) -> Option<Vec<Direction>> {
        match self {
            Self::Vertical => match dir {
                Direction::Left | Direction::Right => Some(vec![Direction::Up, Direction::Down]),
                _ => None,
            },
            Self::Horizontal => match dir {
                Direction::Up | Direction::Down => Some(vec![Direction::Left, Direction::Right]),
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
enum Mirror {
    LR, // /
    RL, // \
}

impl Mirror {
    fn get_next_dir(&self, dir: &Direction) -> Direction {
        use Direction::*;
        match self {
            Self::LR => match dir {
                Right => Up,
                Up => Right,
                Left => Down,
                Down => Left,
            },
            Self::RL => match dir {
                Right => Down,
                Down => Right,
                Left => Up,
                Up => Left,
            },
        }
    }
}

#[derive(Debug)]
enum Thing {
    Splitter(Splitter),
    Mirror(Mirror),
    Nothing,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

#[allow(dead_code)]
fn debug_map(map: &Vec<Vec<Thing>>, energized: &HashMap<(i32, i32), Vec<Direction>>) {
    let height = map.len();
    let width = map[0].len();

    for y in 0..height {
        for x in 0..width {
            match &map[y][x] {
                Thing::Mirror(mirror) => match mirror {
                    Mirror::LR => {
                        print!("/");
                        continue;
                    }
                    Mirror::RL => {
                        print!("\\");
                        continue;
                    }
                },
                Thing::Splitter(splitter) => match splitter {
                    Splitter::Vertical => {
                        print!("|");
                        continue;
                    }
                    Splitter::Horizontal => {
                        print!("-");
                        continue;
                    }
                },
                _ => (),
            }
            if let Some(dir) = energized.get(&(x as i32, y as i32)) {
                if dir.len() > 1 {
                    print!("{}", dir.len());
                    continue;
                }
                print!(
                    "{}",
                    match dir.first().unwrap() {
                        Direction::Right => ">",
                        Direction::Left => "<",
                        Direction::Up => "^",
                        Direction::Down => "v",
                    }
                )
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn process_1(input: &str) -> String {
    use Direction::*;
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Thing::Splitter(Splitter::Vertical),
                    '-' => Thing::Splitter(Splitter::Horizontal),
                    '/' => Thing::Mirror(Mirror::LR),
                    '\\' => Thing::Mirror(Mirror::RL),
                    '.' => Thing::Nothing,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut energized: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();

    let mut posistions = vec![((-1, 0), Right)];

    loop {
        // if no more entries to check
        if posistions.len() == 0 {
            break;
        }
        // if we have already been there or its out of bounds
        let (mut pos, mut dir) = posistions.pop().unwrap();
        let dpos = dir.to_vec();
        pos = (pos.0 + dpos.0, pos.1 + dpos.1);

        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= width
            || pos.1 >= height
            || energized.entry(pos).or_insert(Vec::new()).contains(&dir)
        {
            continue;
        }

        energized.entry(pos).or_insert(Vec::new()).push(dir.clone());
        match &map[pos.1 as usize][pos.0 as usize] {
            Thing::Splitter(splitter) => {
                if let Some(dirs) = splitter.get_next_dirs(&dir) {
                    dirs.iter().for_each(|d| posistions.push((pos, d.clone())));
                    continue;
                }
            }
            Thing::Mirror(mirror) => {
                dir = mirror.get_next_dir(&dir);
            }
            Thing::Nothing => {}
        }
        posistions.push((pos, dir));
    }

    energized
        .iter()
        .filter(|(_, v)| v.len() > 0)
        .count()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    use Direction::*;
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Thing::Splitter(Splitter::Vertical),
                    '-' => Thing::Splitter(Splitter::Horizontal),
                    '/' => Thing::Mirror(Mirror::LR),
                    '\\' => Thing::Mirror(Mirror::RL),
                    '.' => Thing::Nothing,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let starting_pos = (0..height)
        .map(|y| (-1, y))
        .chain((0..height).map(|y| (width, y)))
        .chain((0..width).map(|x| (x, -1)))
        .chain((0..width).map(|x| (x, height)))
        .collect::<Vec<_>>();

    starting_pos
        .into_iter()
        .map(|start_pos| {
            let start_dir = if start_pos.0 == -1 {
                Right
            } else if start_pos.0 == width {
                Left
            } else if start_pos.1 == -1 {
                Down
            } else if start_pos.1 == height {
                Up
            } else {
                unreachable!()
            };

            let mut posistions = vec![(start_pos, start_dir)];
            let mut energized: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();

            loop {
                // if no more entries to check
                if posistions.len() == 0 {
                    break;
                }
                // if we have already been there or its out of bounds
                let (mut pos, mut dir) = posistions.pop().unwrap();
                let dpos = dir.to_vec();
                pos = (pos.0 + dpos.0, pos.1 + dpos.1);

                if pos.0 < 0
                    || pos.1 < 0
                    || pos.0 >= width
                    || pos.1 >= height
                    || energized.entry(pos).or_insert(Vec::new()).contains(&dir)
                {
                    continue;
                }

                energized.entry(pos).or_insert(Vec::new()).push(dir.clone());
                match &map[pos.1 as usize][pos.0 as usize] {
                    Thing::Splitter(splitter) => {
                        if let Some(dirs) = splitter.get_next_dirs(&dir) {
                            dirs.iter().for_each(|d| posistions.push((pos, d.clone())));
                            continue;
                        }
                    }
                    Thing::Mirror(mirror) => {
                        dir = mirror.get_next_dir(&dir);
                    }
                    Thing::Nothing => {}
                }
                posistions.push((pos, dir));
            }

            energized.iter().filter(|(_, v)| v.len() > 0).count()
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_process_1() {
        let res = process_1(INPUT);
        assert_eq!("46", res);
    }

    #[test]
    fn test_process_2() {
        let res = process_2(INPUT);
        assert_eq!("51", res);
    }
}
