use itertools::Itertools;
use std::collections::HashSet;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn process_mirror(lines: &Vec<Vec<char>>) -> HashSet<usize> {
    let mut lines = lines.iter();
    let first_line = lines.next().unwrap();
    let mut cols_to_check = HashSet::new();

    for i in 1..=(first_line.len() - 1) {
        let w = i.min(first_line.len() - i);
        if (0..w).all(|k| first_line.get(i - k - 1).unwrap() == first_line.get(i + k).unwrap()) {
            cols_to_check.insert(i);
        }
    }

    lines.for_each(|line| {
        for col in cols_to_check.clone() {
            let w = col.min(first_line.len() - col);
            if !((0..w).all(|k| line.get(col - k - 1).unwrap() == line.get(col + k).unwrap())) {
                cols_to_check.remove(&col);
            }
        }
    });
    cols_to_check
}

pub fn process_1(input: &str) -> String {
    let patterns = input.split("\n\n");
    patterns
        .map(|pattern| {
            let pattern = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let col_mirror = process_mirror(&pattern).iter().sum::<usize>();
            let t = transpose(pattern);
            let row_mirror = process_mirror(&t).iter().sum::<usize>();
            col_mirror + row_mirror * 100
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let patterns = input.split("\n\n");
    patterns
        .map(|pattern| {
            // loool brute force 🤓🤓
            let pattern = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let og_col_mirror = process_mirror(&pattern).iter().sum::<usize>();
            let (og_row_mirror, pattern) = if og_col_mirror == 0 {
                let t = transpose(pattern);
                let og_row_mirror = process_mirror(&t).iter().sum::<usize>();
                let pattern = transpose(t);
                (og_row_mirror, pattern)
            } else {
                (0, pattern)
            };

            let height = pattern.len();
            let width = pattern.get(0).unwrap().len();

            (0..width)
                .cartesian_product(0..height)
                .map(|(x, y)| {
                    let mut tmp_pattern = pattern.clone();
                    let tmp = *tmp_pattern.get(y).unwrap().get(x).unwrap();
                    *tmp_pattern.get_mut(y).unwrap().get_mut(x).unwrap() = match tmp {
                        '#' => '.',
                        '.' => '#',
                        _ => panic!(),
                    };
                    assert_ne!(tmp, *tmp_pattern.get(y).unwrap().get(x).unwrap());

                    let col_mirror = process_mirror(&tmp_pattern);
                    let col_mirror = col_mirror
                        .into_iter()
                        .filter(|x| x != &og_col_mirror)
                        .next()
                        .unwrap_or(0);

                    return if col_mirror != 0 {
                        col_mirror
                    } else {
                        let t = transpose(tmp_pattern);
                        let row_mirrors = process_mirror(&t);
                        100 * row_mirrors
                            .into_iter()
                            .filter(|x| x != &og_row_mirror)
                            .next()
                            .unwrap_or(0)
                    };
                }).max().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let res = process_1(input);
        assert_eq!("5", res);
    }

    #[test]
    fn test_process_1_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = process_1(input);
        assert_eq!("400", res);
    }

    #[test]
    fn test_process_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = process_1(input);
        assert_eq!("405", res);
    }

    #[test]
    fn test_process_2_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let res = process_2(input);
        assert_eq!("300", res);
    }

    #[test]
    fn test_process_2_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = process_2(input);
        assert_eq!("100", res);
    }

    #[test]
    fn test_process_2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = process_2(input);
        assert_eq!("400", res);
    }
}
