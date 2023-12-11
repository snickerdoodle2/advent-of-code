use std::collections::HashSet;
use itertools::*;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

pub fn process_1(input: &str) -> String {
    let width = input.lines().next().unwrap().len();
    let mut empty_cols = vec![true; width];
    let mut empty_rows = Vec::new();
    let mut galaxies = Vec::new();
    input.lines().enumerate().for_each(|(line_idx, line)| {
        let mut is_empty = true;
        line.chars()
            .enumerate()
            .for_each(|(char_idx, char)| match char {
                '.' => (),
                '#' => {
                    is_empty = false;
                    empty_cols[char_idx] = false;
                    galaxies.push(Point {
                        x: char_idx,
                        y: line_idx,
                    });
                }
                _ => panic!(":("),
            });
        if is_empty {
            empty_rows.push(line_idx);
        }
    });

    let empty_cols = empty_cols
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            true => Some(i),
            false => None,
        })
        .collect::<Vec<_>>();

    let galaxies = galaxies.iter().map(|Point { mut x, mut y }| {
        x += empty_cols.iter().filter(|c| c < &&x).count();
        y += empty_rows.iter().filter(|r| r < &&y).count();
        Point { x, y }
    }).collect::<Vec<_>>();

    let pairs = galaxies.into_iter().combinations(2).map(|mut v| {
        v.sort();
        (v[0].clone(), v[1].clone())
    }).collect::<HashSet<_>>();

    pairs.iter().map(|(a, b)| {
        (a.x as i64 - b.x as i64).abs() + (a.y as i64 - b.y as i64).abs()
    }).sum::<i64>().to_string()
}

pub fn process_2(input: &str, dx: usize) -> String {
    let width = input.lines().next().unwrap().len();
    let mut empty_cols = vec![true; width];
    let mut empty_rows = Vec::new();
    let mut galaxies = Vec::new();
    input.lines().enumerate().for_each(|(line_idx, line)| {
        let mut is_empty = true;
        line.chars()
            .enumerate()
            .for_each(|(char_idx, char)| match char {
                '.' => (),
                '#' => {
                    is_empty = false;
                    empty_cols[char_idx] = false;
                    galaxies.push(Point {
                        x: char_idx,
                        y: line_idx,
                    });
                }
                _ => panic!(":("),
            });
        if is_empty {
            empty_rows.push(line_idx);
        }
    });

    let empty_cols = empty_cols
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            true => Some(i),
            false => None,
        })
        .collect::<Vec<_>>();

    let galaxies = galaxies.iter().map(|Point { mut x, mut y }| {
        x += empty_cols.iter().filter(|c| c < &&x).count() * (dx - 1);
        y += empty_rows.iter().filter(|r| r < &&y).count() * (dx - 1);
        Point { x, y }
    }).collect::<Vec<_>>();

    let pairs = galaxies.into_iter().combinations(2).map(|mut v| {
        v.sort();
        (v[0].clone(), v[1].clone())
    }).collect::<HashSet<_>>();

    pairs.iter().map(|(a, b)| {
        (a.x as i64 - b.x as i64).abs() + (a.y as i64 - b.y as i64).abs()
    }).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_process_1() {
        let res = process_1(INPUT);
        assert_eq!("374", res);
    }

    #[test]
    fn test_process_2_1() {
        let res = process_2(INPUT, 10);
        assert_eq!("1030", res);
    }
    
    #[test]
    fn test_process_2_2() {
        let res = process_2(INPUT, 100);
        assert_eq!("8410", res);
    }
}
