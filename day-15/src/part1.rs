#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Entity {
    Wall,
    Box,
    Robot,
}

#[derive(Debug)]
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
                .map(|x| match x {
                    '#' => Some(Entity::Wall),
                    'O' => Some(Entity::Box),
                    '@' => Some(Entity::Robot),
                    '.' => None,
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

fn move_entity(map: &mut Map, x: isize, y: isize, dx: isize, dy: isize) -> Option<()> {
    let (new_x, new_y) = (x + dx, y + dy);

    let Some(next_tile) = map[new_y as usize][new_x as usize] else {
        map[new_y as usize][new_x as usize] = Some(map[y as usize][x as usize]?);
        map[y as usize][x as usize] = None;
        return Some(());
    };

    if next_tile == Entity::Wall {
        return None;
    }

    move_entity(map, new_x, new_y, dx, dy)?;

    map[new_y as usize][new_x as usize] = Some(map[y as usize][x as usize]?);
    map[y as usize][x as usize] = None;
    return Some(());
}

fn print_map(map: &Map) {
    for row in map {
        for ent in row {
            let c = match ent {
                Some(Entity::Wall) => '#',
                Some(Entity::Box) => 'O',
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
        let (dx, dy) = dir.to_vec();
        let res = move_entity(&mut map, x, y, dx, dy);
        if res.is_some() {
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
                Some(Entity::Box) => {
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
        r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#,
        "2028"
    )]
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
        "10092"
    )]
    fn test_part1(#[case] input: &str, #[case] expected: String) {
        assert_eq!(expected, process(input));
    }
}
