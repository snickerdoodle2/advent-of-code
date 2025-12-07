use std::collections::HashSet;

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

pub fn process(input: &str) -> String {
    let (start, cells) = parse(input);
    let mut tachyons = HashSet::<usize>::new();
    let mut res = 0;
    tachyons.insert(start);

    for line in cells {
        let mut to_add = vec![];
        let mut to_remove = vec![];
        for tachyon in tachyons.iter() {
            if line[*tachyon] == Cell::Splitter {
                to_add.push(tachyon - 1);
                to_add.push(tachyon + 1);
                to_remove.push(*tachyon);
                res += 1;
            }
        }

        tachyons.extend(to_add.into_iter());
        to_remove.iter().for_each(|c| {
            tachyons.remove(c);
        });
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!("21", process(input));
    }
}
