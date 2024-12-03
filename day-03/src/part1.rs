use nom::{
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    multi::many_till,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Mul(u16, u16);

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

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (input, _) = tag("mul")(input)?;
    let (input, x) = delimited(
        char('('),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    )(input)?;

    Ok((input, Mul(x.0, x.1)))
}

fn parse(mut input: &str) -> IResult<&str, Vec<Mul>> {
    let mut muls: Vec<Mul> = vec![];
    while let Ok(res) = many_till(take(1 as usize), parse_mul)(input) {
        muls.push(res.1 .1);
        input = res.0;
    }
    Ok((input, muls))
}

pub fn process(input: &str) -> String {
    let (_, ops) = parse(input).unwrap();
    ops.iter()
        .map(|m| m.0 as u32 * m.1 as u32)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input));
    }
}
