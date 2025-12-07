use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Splitter,
    Empty,
}

fn parse(input: &str) -> (usize, Vec<Vec<Cell>>) {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let (start, _) = first.chars().enumerate().find(|(_, x)| *x == 'S').unwrap();

    let cells = lines
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '^' => Cell::Splitter,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (start, cells)
}

fn iteration(
    cells: &[Vec<Cell>],
    line: usize,
    col: usize,
    memoize: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(res) = memoize.get(&(line, col)) {
        return *res;
    }
    let Some(cur_line) = cells.get(line) else {
        return 1;
    };

    let res = match cur_line[col] {
        Cell::Splitter => {
            iteration(cells, line + 1, col - 1, memoize)
                + iteration(cells, line + 1, col + 1, memoize)
        }
        Cell::Empty => iteration(cells, line + 1, col, memoize),
    };

    memoize.insert((line, col), res);

    res
}

pub fn process(input: &str) -> String {
    let (start, cells) = parse(input);

    let mut memo = HashMap::new();
    iteration(&cells, 0, start, &mut memo).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        assert_eq!("40", process(input));
    }
}
