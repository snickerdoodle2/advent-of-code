use nom::{
    character::complete::{self, char},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::ops::RangeInclusive;

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (lhs, rhs)) = separated_pair(complete::u64, char('-'), complete::u64)(input)?;
    Ok((input, lhs..=rhs))
}

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    separated_list1(char(','), parse_range)(input)
        .expect("should parse")
        .1
}

fn is_invalid(x: u64) -> bool {
    let digits = x.ilog10() + 1;
    if digits % 2 == 1 {
        return false;
    }
    let half_size = 10u64.pow(digits / 2);
    let lhs = x / half_size;
    let rhs = x % half_size;

    lhs == rhs
}

pub fn process(input: &str) -> String {
    parse(input)
        .into_iter()
        .flat_map(|subrange| subrange.filter(|x| is_invalid(*x)))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input));
    }
}
