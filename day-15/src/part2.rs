use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Entity {
    Wall,
    BoxL,
    BoxR,
    Robot,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vec(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

type Map = Vec<Vec<Option<Entity>>>;

fn parse(input: &str) -> (Map, Vec<Direction>) {
    let mut input = input.split("\n\n");
    let map: Map = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|x| match x {
                    '#' => vec![Some(Entity::Wall), Some(Entity::Wall)],
                    'O' => vec![Some(Entity::BoxL), Some(Entity::BoxR)],
                    '@' => vec![Some(Entity::Robot), None],
                    '.' => vec![None, None],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let dirs = input
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|x| match x {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (map, dirs)
}

fn robot_pos(map: &Map) -> (isize, isize) {
    for (y, row) in map.iter().enumerate() {
        for (x, entity) in row.iter().enumerate() {
            if *entity == Some(Entity::Robot) {
                return (x as isize, y as isize);
            }
        }
    }

    unreachable!()
}

// THIS RUNS ONLY IF DIRECTION IS UP OR DOWN
fn move_box(
    map: &mut Map,
    x: isize,
    y: isize,
    dir: Direction,
) -> Option<HashSet<(isize, isize, Entity)>> {
    let this = map[y as usize][x as usize]?;
    let other_x = match this {
        Entity::BoxL => x + 1,
        Entity::BoxR => x - 1,
        _ => unreachable!(),
    };

    let other = map[y as usize][other_x as usize]?;

    let (_, dy) = dir.to_vec();
    let new_y = y + dy;

    let next = map[new_y as usize][x as usize];
    let other_next = map[new_y as usize][other_x as usize];

    if next.is_none() && other_next.is_none() {
        let mut res = HashSet::new();
        res.insert((x, y, this));
        res.insert((other_x, y, other));
        return Some(res);
    }

    if next == Some(Entity::Wall) || other_next == Some(Entity::Wall) {
        return None;
    }

    if next == Some(this) {
        let mut res = move_box(map, x, new_y, dir)?;
        res.insert((x, y, this));
        res.insert((other_x, y, other));
        return Some(res);
    }

    let res = {
        if next.is_none() {
            Some(HashSet::new())
        } else {
            move_box(map, x, new_y, dir)
        }
    };
    let res_other = {
        if other_next.is_none() {
            Some(HashSet::new())
        } else {
            move_box(map, other_x, new_y, dir)
        }
    };

    if res.is_some() && res_other.is_some() {
        let mut res = res.unwrap();
        res.extend(res_other.unwrap());
        res.insert((x, y, this));
        res.insert((other_x, y, other));
        return Some(res);
    }

    None
}

fn move_entity(map: &mut Map, x: isize, y: isize, dir: Direction) -> Option<()> {
    let this = map[y as usize][x as usize]?;

    if (dir == Direction::Up || dir == Direction::Down)
        && (this == Entity::BoxL || this == Entity::BoxR)
    {
        let to_move = move_box(map, x, y, dir)?;
        let mut to_remove: HashSet<(usize, usize)> = to_move
            .clone()
            .into_iter()
            .map(|(x, y, _)| (x as usize, y as usize))
            .collect();

        for (x, y, e) in to_move {
            let (_, dy) = dir.to_vec();
            let x = x as usize;
            let new_y = (y + dy) as usize;
            map[new_y][x] = Some(e);
            to_remove.remove(&(x, new_y));
        }

        for (x, y) in to_remove {
            map[y][x] = None;
        }
        return Some(());
    }

    let (dx, dy) = dir.to_vec();
    let (new_x, new_y) = (x + dx, y + dy);

    let Some(next_tile) = map[new_y as usize][new_x as usize] else {
        map[new_y as usize][new_x as usize] = Some(this);
        map[y as usize][x as usize] = None;
        return Some(());
    };

    if next_tile == Entity::Wall {
        return None;
    }

    move_entity(map, new_x, new_y, dir)?;

    map[new_y as usize][new_x as usize] = Some(this);
    map[y as usize][x as usize] = None;
    return Some(());
}

fn print_map(map: &Map) {
    for row in map {
        for ent in row {
            let c = match ent {
                Some(Entity::Wall) => '#',
                Some(Entity::BoxL) => '[',
                Some(Entity::BoxR) => ']',
                Some(Entity::Robot) => '@',
                None => '.',
            };
            print!("{c}");
        }
        println!();
    }
}

pub fn process(input: &str) -> String {
    let (mut map, dirs) = parse(input);
    let (mut x, mut y) = robot_pos(&map);

    for dir in dirs {
        let res = move_entity(&mut map, x, y, dir);
        if res.is_some() {
            let (dx, dy) = dir.to_vec();
            x += dx;
            y += dy;
        }
    }
    let height = map.len();
    let width = map[0].len();

    let mut res = 0;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            match map[y][x] {
                Some(Entity::BoxL) => {
                    res += 100 * y + x;
                }
                _ => continue,
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
        "9021"
    )]
    fn test_part2(#[case] input: &str, #[case] expected: String) {
        assert_eq!(expected, process(input));
    }
}
