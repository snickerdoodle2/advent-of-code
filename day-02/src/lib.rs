use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult,
};

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<(usize, usize, usize)>,
}

#[derive(Debug)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, amount) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    let cube = match color {
        "red" => Cube::Red(amount.parse().unwrap()),
        "green" => Cube::Green(amount.parse().unwrap()),
        "blue" => Cube::Blue(amount.parse().unwrap()),
        _ => unreachable!(),
    };
    Ok((input, cube))
}

fn parse_set(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    let mut res = (0, 0, 0);
    for cube in cubes {
        match cube {
            Cube::Red(amount) => res.0 += amount,
            Cube::Green(amount) => res.1 += amount,
            Cube::Blue(amount) => res.2 += amount,
        }
    }
    Ok((input, res))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, subsets) = separated_list1(tag("; "), parse_set)(input)?;

    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            subsets,
        },
    ))
}

pub fn process_1(input: &str) -> String {
    let max = (12, 13, 14);

    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).unwrap();
            game
        })
        .collect();

    let mut res = 0;

    for game in &games {
        let test = game
            .subsets
            .iter()
            .all(|(r, g, b)| r <= &max.0 && g <= &max.1 && b <= &max.2);
        if test {
            res += game.id;
        }
    }

    res.to_string()
}

pub fn process_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).unwrap();
            let max_r = game.subsets.iter().map(|(r, _, _)| r).max().unwrap();
            let max_g = game.subsets.iter().map(|(_, g, _)| g).max().unwrap();
            let max_b = game.subsets.iter().map(|(_, _, b)| b).max().unwrap();
            max_r * max_g * max_b
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_process_1() {
        let res = process_1(INPUT);
        assert_eq!("8", res);
    }

    #[test]
    fn test_process_2() {
        let res = process_2(INPUT);
        assert_eq!("2286", res);
    }
}
