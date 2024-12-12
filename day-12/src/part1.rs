use std::collections::HashMap;

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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
) -> (u32, u32) {
    if !in_bounds(x, y, width, height) || visited[y as usize][x as usize] {
        return (0, 0);
    }

    let x = x as usize;
    let y = y as usize;

    visited[y][x] = true;

    let field = map[y][x];

    let mut area = 1;
    let mut perimeter = 0;
    for (dx, dy) in DIRS {
        let x = x as isize + dx;
        let y = y as isize + dy;
        if !in_bounds(x, y, width, height) || map[y as usize][x as usize] != field {
            perimeter += 1;
        } else {
            let (new_area, new_perimeter) = calculate(x, y, width, height, map, visited);
            area += new_area;
            perimeter += new_perimeter;
        }
    }

    (area, perimeter)
}

pub fn process(input: &str) -> String {
    let map = parse(input);
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    let mut visited = vec![vec![false; width as usize]; height as usize];

    let mut result = 0;

    for y in 0..height {
        for x in 0..width {
            let (area, perimeter) = calculate(x, y, width, height, &map, &mut visited);
            result += area * perimeter;
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
        "140"
    )]
    #[case(
        "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        "772"
    )]
    #[case(
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        "1930"
    )]
    fn test_part1(#[case] input: &str, #[case] output: String) {
        assert_eq!(output, process(input));
    }
}
