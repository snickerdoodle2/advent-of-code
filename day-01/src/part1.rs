pub fn process(input: &str) -> String {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let mut splitted = line.split_whitespace();
        let left_ = splitted.next().unwrap().parse().unwrap();
        let right_ = splitted.next().unwrap().parse().unwrap();
        left.push(left_);
        right.push(right_);
    });
    left.sort();
    right.sort();

    let res: i32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| (r - l).abs())
        .sum();

    format!("{}", res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!("11", process(input));
    }
}
