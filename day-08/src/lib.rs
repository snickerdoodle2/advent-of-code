use std::collections::{BTreeMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use num::Integer;

#[derive(Debug)]
struct Instruction<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_instruction(input: &str) -> IResult<&str, (&str, Instruction)> {
    let (input, from) = terminated(alphanumeric1, tag(" = "))(input)?;
    let (input, (left, right)) = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(input)?;
    Ok((input, (from, Instruction { left, right })))
}

fn parse_input(input: &str) -> IResult<&str, (VecDeque<char>, BTreeMap<&str, Instruction>)> {
    let (input, chars) = alpha1(input)?;
    let chars = chars.chars().collect::<VecDeque<_>>();

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;

    let (input, instructions) = separated_list1(newline, parse_instruction)(input)?;

    Ok((input, (chars, instructions.into_iter().collect())))
}

pub fn process_1(input: &str) -> String {
    let (_, (mut chars, instructions)) = parse_input(input).unwrap();

    let mut cur = "AAA";
    let mut counter = 0;

    loop {
        counter += 1;
        let next_hop = chars.pop_front().unwrap();
        let instruction = instructions.get(cur).unwrap();
        cur = match next_hop {
            'R' => instruction.right,
            'L' => instruction.left,
            _ => panic!("huh"),
        };
        if cur == "ZZZ" {
            return counter.to_string();
        }
        chars.push_back(next_hop);
    }
}

fn traverse(
    start: &str,
    instructions: &BTreeMap<&str, Instruction>,
    mut chars: VecDeque<char>,
) -> u64 {
    let mut counter = 0;

    let mut cur = start;

    loop {
        counter += 1;
        let next_hop = chars.pop_front().unwrap();
        let instruction = instructions.get(cur).unwrap();
        cur = match next_hop {
            'R' => instruction.right,
            'L' => instruction.left,
            _ => panic!("huh"),
        };
        if cur.ends_with("Z") {
            return counter;
        }
        chars.push_back(next_hop);
    }
}

pub fn process_2(input: &str) -> String {
    let (_, (chars, instructions)) = parse_input(input).unwrap();

    let mut solutions = instructions
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|start| traverse(start, &instructions, chars.clone())).collect::<Vec<_>>();

    solutions.sort();

    let mut res = 1;
    for sol in &solutions {
        let tmp = res.lcm(sol);
        println!("lcm({}, {}) = {}", sol, res, tmp);
        res = tmp;
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let res = process_1(input);
        assert_eq!("2", res);
    }

    #[test]
    fn test_process_1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let res = process_1(input);
        assert_eq!("6", res);
    }

    #[test]
    fn test_process_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let res = process_2(input);
        assert_eq!("6", res);
    }
}
