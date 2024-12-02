#[derive(Clone, Copy, PartialEq, Eq)]
enum Type {
    Decreasing,
    Increasing,
}

impl From<i32> for Type {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self::Decreasing
        } else {
            Self::Increasing
        }
    }
}

pub fn process(input: &str) -> String {
    let res = input
        .lines()
        .filter_map(|line| {
            let levels: Vec<_> = line.split_whitespace().into_iter().collect();
            'outer: for to_skip in 0..levels.len() {
                let mut levels = levels.clone().into_iter().enumerate().filter_map(|(i, x)| {
                    if i == to_skip {
                        None
                    } else {
                        Some(x)
                    }
                });
                let mut prev_level: i32 = levels.next().unwrap().parse().unwrap();
                let mut diff_type: Option<Type> = None;

                for level in levels {
                    let level: i32 = level.parse().unwrap();
                    let diff = prev_level - level;
                    if !(1..=3).contains(&diff.abs()) {
                        continue 'outer;
                    }

                    let new_diff_type: Type = diff.into();

                    if let Some(diff_type) = diff_type {
                        if diff_type != new_diff_type {
                            continue 'outer;
                        }
                    } else {
                        diff_type = Some(new_diff_type);
                    }
                    prev_level = level;
                }

                return Some(());
            }

            None
        })
        .count();

    format!("{}", res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!("4", process(input));
    }
}
