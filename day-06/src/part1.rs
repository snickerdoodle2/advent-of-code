use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, space0},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug)]
enum Problem {
    Add(Vec<u64>),
    Mult(Vec<u64>),
}

fn parse_num_line(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(many1(preceded(space0, complete::u64)), newline)(input)
}

fn parse_op_line(input: &str) -> IResult<&str, Vec<&str>> {
    many1(preceded(space0, alt((tag("+"), tag("*")))))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    let (input, nums) = many1(parse_num_line)(input)?;
    let mut items: Vec<Vec<u64>> = vec![Vec::with_capacity(nums.len()); nums[0].len()];

    for row in nums {
        for (i, x) in row.iter().enumerate() {
            items[i].push(*x);
        }
    }

    let mut res = vec![];
    let (input, ops) = parse_op_line(input)?;

    for (op, items) in ops.into_iter().zip(items.into_iter()) {
        let problem = match op {
            "+" => Problem::Add(items),
            "*" => Problem::Mult(items),
            _ => unreachable!(),
        };

        res.push(problem);
    }

    Ok((input, res))
}

pub fn process(input: &str) -> String {
    let (_, problems) = parse(input).unwrap();

    problems
        .iter()
        .map(|p| match p {
            Problem::Add(items) => items.iter().sum::<u64>(),
            Problem::Mult(items) => items.iter().product::<u64>(),
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +"#;
        assert_eq!("4277556", process(input));
    }
}
