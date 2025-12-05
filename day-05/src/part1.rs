use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) = terminated(
        separated_pair(complete::u64, tag("-"), complete::u64),
        newline,
    )(input)?;

    Ok((input, start..=end))
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<RangeInclusive<u64>>)> {
    let (input, fresh) = terminated(many1(parse_range), newline)(input)?;
    let (input, items) = separated_list1(newline, complete::u64)(input)?;
    Ok((input, (items, fresh)))
}

fn is_fresh(item: u64, fresh: &[RangeInclusive<u64>]) -> bool {
    fresh.iter().any(|range| range.contains(&item))
}

pub fn process(input: &str) -> String {
    let (_, (items, fresh)) = parse(input).expect("should parse");

    items
        .into_iter()
        .filter(|x| is_fresh(*x, &fresh))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        assert_eq!("3", process(input));
    }
}
