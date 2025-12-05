use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::many1,
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

fn parse(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, fresh) = terminated(many1(parse_range), newline)(input)?;
    Ok((input, fresh))
}

pub fn process(input: &str) -> String {
    let (_, mut fresh) = parse(input).expect("should parse");

    fresh.sort_by_key(|x| *x.start());
    let mut fresh = fresh.iter();
    let first = fresh.next().unwrap();
    let mut start = *first.start();
    let mut end = *first.end();
    let mut res = 0;

    for range in fresh {
        if *range.start() <= end + 1 {
            if *range.end() > end {
                end = *range.end();
            }
        } else {
            res += (start..=end).count();
            start = *range.start();
            end = *range.end();
        }
    }

    res += (start..=end).count();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!("14", process(input));
    }
}
