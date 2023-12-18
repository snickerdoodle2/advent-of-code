use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, anychar, newline},
    multi::{count, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i32,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    use Direction::*;
    let (input, direction) = terminated(
        alt((
            tag("U").map(|_| Up),
            tag("R").map(|_| Right),
            tag("D").map(|_| Down),
            tag("L").map(|_| Left),
        )),
        tag(" "),
    )(input)?;
    let (input, amount) = terminated(complete::i32, tag(" "))(input)?;
    let (input, _color) = delimited(tag("(#"), alphanumeric1, tag(")"))(input)?;
    Ok((input, Move { direction, amount }))
}

fn parse_move_2(input: &str) -> IResult<&str, Move> {
    use Direction::*;
    let (input, _) = terminated(alt((tag("U"), tag("R"), tag("D"), tag("L"))), tag(" "))(input)?;
    let (input, _) = terminated(complete::i32, tag(" "))(input)?;
    let (input, (amount, direction)) = delimited(
        tag("(#"),
        tuple((
            count(anychar, 5),
            alt((
                tag("0").map(|_| Right),
                tag("1").map(|_| Down),
                tag("2").map(|_| Left),
                tag("3").map(|_| Up),
            )),
        )),
        tag(")"),
    )(input)?;
    Ok((
        input,
        Move {
            direction,
            amount: i32::from_str_radix(amount.iter().collect::<String>().as_str(), 16).unwrap(),
        },
    ))
}

#[allow(dead_code)]
fn debug_map(map: &HashSet<(i32, i32)>) {
    let min_y = map.iter().map(|(_, y)| y).min().unwrap().clone();
    let max_y = map.iter().map(|(_, y)| y).max().unwrap().clone();

    let min_x = map.iter().map(|(x, _)| x).min().unwrap().clone();
    let max_x = map.iter().map(|(x, _)| x).max().unwrap().clone();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn flood_fill(map: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
    if map.contains(&(x, y)) {
        return;
    }
    map.insert((x, y));
    flood_fill(map, x - 1, y);
    flood_fill(map, x + 1, y);
    flood_fill(map, x, y - 1);
    flood_fill(map, x, y + 1);
}

pub fn process_1(input: &str) -> String {
    let (_, moves) = separated_list1(newline, parse_move)(input).unwrap();
    let mut pos = (0, 0);
    let mut map = moves
        .iter()
        .flat_map(|move_| {
            let v = move_.direction.to_vec();
            let (x, y) = pos;
            let new_pos = (x + v.0 * move_.amount, y + v.1 * move_.amount);
            let x_range = if new_pos.0 > x {
                x..=new_pos.0
            } else {
                new_pos.0..=x
            };

            let y_range = if new_pos.1 > y {
                y..=new_pos.1
            } else {
                new_pos.1..=y
            };
            pos = new_pos;
            x_range.into_iter().cartesian_product(y_range).collect_vec()
        })
        .collect::<HashSet<_>>();

    flood_fill(&mut map, 1, 1);

    (map.iter().count()).to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, moves) = separated_list1(newline, parse_move_2)(input).unwrap();
    let mut pos = (0, 0);
    let mut map = moves
        .iter()
        .map(|move_| {
            let v = move_.direction.to_vec();
            let (x, y) = pos;
            let (x, y) = (x as i64, y as i64);
            let new_pos = (
                x + v.0 as i64 * move_.amount as i64,
                y + v.1 as i64 * move_.amount as i64,
            );
            pos = new_pos;
            new_pos
        })
        .collect::<Vec<_>>();

    map.insert(0, (0, 0));

    let res = map
        .windows(2)
        .map(|i| i[0].0 * i[1].1 - i[0].1 * i[1].0)
        .sum::<i64>();

    let outer = moves.iter().fold(0, |acc, e| acc + e.amount);

    ((res + outer as i64 + 2) / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let res = process_1(input);
        assert_eq!("62", res);
    }

    #[test]
    fn test_process_2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let res = process_2(input);
        assert_eq!("952408144115", res);
    }
}
