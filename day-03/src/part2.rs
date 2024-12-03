use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    multi::many_till,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
enum Instruction {
    Mul(u16, u16),
    Do,
    Dont,
}

fn parse_number(input: &str) -> IResult<&str, u16> {
    let (input, num) = digit1(input)?;
    if (1..=3).contains(&num.len()) {
        Ok((input, num.parse().expect("Should parse")))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )))
    }
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, x) = delimited(
        char('('),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    )(input)?;

    Ok((input, Instruction::Mul(x.0, x.1)))
}

fn parse_dos(input: &str) -> IResult<&str, Instruction> {
    let (input, res) = alt((tag("do()"), tag("don't()")))(input)?;

    let res = match res {
        "do()" => Instruction::Do,
        "don't()" => Instruction::Dont,
        _ => unreachable!(),
    };

    Ok((input, res))
}

fn parse(mut input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut muls: Vec<Instruction> = vec![];
    while let Ok(res) = many_till(take(1 as usize), alt((parse_mul, parse_dos)))(input) {
        muls.push(res.1 .1);
        input = res.0;
    }
    Ok((input, muls))
}

pub fn process(input: &str) -> String {
    let (_, ops) = parse(input).unwrap();
    let mut active = true;
    let mut sum: u32 = 0;

    for op in ops {
        match op {
            Instruction::Mul(a, b) => {
                if active {
                    sum += a as u32 * b as u32
                }
            }
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input));
    }
}
