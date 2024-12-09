use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

type Equation = (i64, Vec<i64>);

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, answer) = complete::i64(input)?;
    let (input, parts) = preceded(tag(": "), separated_list1(tag(" "), complete::i64))(input)?;

    Ok((input, (answer, parts)))
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(newline, parse_equation)(input)
}

// NOTE: WORKS BACKWARDS
fn check_equation(target: i64, nums: &[i64], idx: usize) -> bool {
    if target < 0 {
        return false;
    }
    if idx == 0 {
        return target == nums[0];
    }

    let num = nums[idx];
    if target % num == 0 {
        if check_equation(target / num, nums, idx - 1) {
            return true;
        }
    }

    check_equation(target - num, nums, idx - 1)
}

pub fn process(input: &str) -> String {
    let (_, equations) = parse(input).unwrap();

    let res: i64 = equations
        .into_iter()
        .filter_map(|(ans, nums)| {
            if check_equation(ans, &nums, nums.len() - 1) {
                Some(ans)
            } else {
                None
            }
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_recursive() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!("3749", process(input));
    }
}
