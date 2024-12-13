use std::collections::HashMap;

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

type Cache = HashMap<(u64, u64), Option<u32>>;

fn calculate(
    x: u64,
    y: u64,
    a_moves: u8,
    b_moves: u8,
    game: &Game,
    cache: &mut Cache,
) -> Option<u32> {
    if a_moves > 100 || b_moves > 100 {
        return None;
    }

    if x > game.prize.0 || y > game.prize.1 {
        return None;
    }

    if x == game.prize.0 && y == game.prize.1 {
        return Some(0);
    }

    if let Some(cached) = cache.get(&(x, y)) {
        return *cached;
    }

    let a_move = calculate(
        x + game.a.0,
        y + game.a.1,
        a_moves + 1,
        b_moves,
        game,
        cache,
    )
    .map(|x| -(x as i32) - 3);
    let b_move = calculate(
        x + game.b.0,
        y + game.b.1,
        a_moves,
        b_moves + 1,
        game,
        cache,
    )
    .map(|x| -(x as i32) - 1);

    let res = a_move.max(b_move).map(|x| -x as u32);
    cache.insert((x, y), res);

    res
}

pub fn process(input: &str) -> String {
    let (_, games) = many1(parse_game)(input).unwrap();

    games
        .into_iter()
        .filter_map(|game| {
            let mut cache = Cache::new();
            calculate(0, 0, 0, 0, &game, &mut cache)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!("480", process(input));
    }
}
