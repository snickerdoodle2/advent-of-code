use nom::{
    character::{
        complete::{self, newline},
        streaming::one_of,
    },
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Turn {
    Left(i16),
    Right(i16),
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::Left(x) => write!(f, "L{x}"),
            Turn::Right(x) => write!(f, "R{x}"),
        }
    }
}

fn parse_line(line: &str) -> IResult<&str, Turn> {
    let (line, dir) = one_of("LR")(line)?;
    let (line, amount) = complete::i16(line)?;

    let turn = match dir {
        'L' => Turn::Left(amount),
        'R' => Turn::Right(amount),
        _ => unreachable!(),
    };
    Ok((line, turn))
}

pub fn process(input: &str) -> String {
    let (_, turns) = separated_list1(newline, parse_line)(input.trim()).unwrap();
    let mut dial = 50;
    let mut res = 0;

    for turn in turns {
        match turn {
            Turn::Left(amount) => {
                dial -= amount;
            }
            Turn::Right(amount) => {
                dial += amount;
            }
        }

        dial = dial.rem_euclid(100);
        if dial == 0 {
            res += 1;
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!("3", process(input));
    }
}
