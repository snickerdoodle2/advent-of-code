use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn to_vec(&self) -> (i8, i8) {
        match self {
            Direction::N => (0, 1),
            Direction::NE => (1, 1),
            Direction::E => (1, 0),
            Direction::SE => (1, -1),
            Direction::S => (0, -1),
            Direction::SW => (-1, -1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, 1),
        }
    }
}

fn find_xmas(
    data: &Vec<Vec<char>>,
    mut remaining: impl Iterator<Item = char>,
    x: i64,
    y: i64,
    direction: Direction,
) -> Option<()> {
    let Some(to_find) = remaining.next() else {
        return Some(());
    };

    let cur = data.get(y as usize)?.get(x as usize)?;

    if *cur != to_find {
        return None;
    }

    let delta = direction.to_vec();
    let (x, y) = (x + delta.0 as i64, y + delta.1 as i64);

    return find_xmas(data, remaining, x, y, direction);
}

pub fn process(input: &str) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut res = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'X' {
                res += Direction::iter()
                    .filter_map(|dir| find_xmas(&input, "XMAS".chars(), x as i64, y as i64, dir))
                    .count();
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_part1_2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!("18", process(input));
    }
}
