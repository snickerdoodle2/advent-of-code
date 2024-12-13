use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, newline},
    multi::{many0, many1},
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
struct Pos(u64, u64);

#[derive(Debug)]
struct Game {
    a: Pos,
    b: Pos,
    prize: Pos,
}

fn parse_button(input: &str) -> IResult<&str, Pos> {
    let (input, _) = delimited(tag("Button "), anychar, tag(": "))(input)?;
    let (input, x) = delimited(tag("X+"), complete::u64, tag(", "))(input)?;
    let (input, y) = delimited(tag("Y+"), complete::u64, newline)(input)?;

    Ok((input, Pos(x, y)))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, a) = parse_button(input)?;
    let (input, b) = parse_button(input)?;
    let (input, x) = delimited(tag("Prize: X="), complete::u64, tag(", Y="))(input)?;
    let (input, y) = terminated(complete::u64, many0(newline))(input)?;

    Ok((
        input,
        Game {
            a,
            b,
            prize: Pos(x, y),
        },
    ))
}

pub fn process(input: &str, error: u64) -> String {
    let games = many1(parse_game)(input)
        .unwrap()
        .1
        .into_iter()
        .map(|mut x| {
            x.prize.0 += error;
            x.prize.1 += error;
            x
        });

    games
        .filter_map(|game| {
            let (x, y) = (game.prize.0 as f64, game.prize.1 as f64);
            let (ax, ay) = (game.a.0 as f64, game.a.1 as f64);
            let (bx, by) = (game.b.0 as f64, game.b.1 as f64);

            let ayax = ay / ax;

            let b_count = (y - ayax * x) / (by - ayax * bx);
            let a_count = (y - by * b_count) / ay;

            let a_count = a_count.round() as u64;
            let b_count = b_count.round() as u64;

            // Sanity check
            if a_count * game.a.0 + b_count * game.b.0 == game.prize.0
                && a_count * game.a.1 + b_count * game.b.1 == game.prize.1
            {
                Some(a_count * 3 + b_count)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input, 0));
    }
}
