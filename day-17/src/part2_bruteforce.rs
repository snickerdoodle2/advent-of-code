use std::cmp::Reverse;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, char, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    cur_instruction: usize,
    output: Vec<u8>,
}

impl Computer {
    fn new(a: u64, b: u64, c: u64) -> Self {
        Self {
            a,
            b,
            c,
            cur_instruction: 0,
            output: Vec::new(),
        }
    }

    fn operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => u64::MAX,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    #[allow(dead_code)]
    Bxc(u8),
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

impl Instruction {
    fn execute(&self, computer: &mut Computer) {
        let mut next_instruction: Option<usize> = None;
        match self {
            Instruction::Adv(operand) => {
                let denominator: u64 = 2_u64.pow(computer.operand(*operand) as u32);
                let res = computer.a / denominator;
                computer.a = res;
            }
            Instruction::Bxl(operand) => {
                let operand = computer.operand(*operand);
                computer.b ^= operand;
            }
            Instruction::Bst(operand) => {
                let operand = computer.operand(*operand);
                computer.b = operand % 8;
            }
            Instruction::Jnz(operand) => {
                if computer.a != 0 {
                    let operand = computer.operand(*operand);
                    next_instruction = Some(operand as usize);
                }
            }
            Instruction::Bxc(_) => {
                let res = computer.b ^ computer.c;
                computer.b = res;
            }
            Instruction::Out(operand) => {
                let res = computer.operand(*operand) % 8;
                computer.output.push(res as u8);
            }
            Instruction::Bdv(operand) => {
                let denominator: u64 = 2_u64.pow(computer.operand(*operand) as u32);
                let res = computer.a / denominator;
                computer.b = res;
            }
            Instruction::Cdv(operand) => {
                let denominator: u64 = 2_u64.pow(computer.operand(*operand) as u32);
                let res = computer.a / denominator;
                computer.c = res;
            }
        }
        computer.cur_instruction = next_instruction.unwrap_or_else(|| computer.cur_instruction + 1);
    }
}

fn parse_instructions(ins: &[u8]) -> Vec<Instruction> {
    ins.into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut i| {
            let opcode = i.next().unwrap();
            let operand = i.next().unwrap();
            let operand = *operand;
            match opcode {
                0 => Instruction::Adv(operand),
                1 => Instruction::Bxl(operand),
                2 => Instruction::Bst(operand),
                3 => Instruction::Jnz(operand),
                4 => Instruction::Bxc(operand),
                5 => Instruction::Out(operand),
                6 => Instruction::Bdv(operand),
                7 => Instruction::Cdv(operand),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, ((u64, u64), Vec<u8>)> {
    let (input, _) = delimited(tag("Register A: "), complete::u64, many1(newline))(input)?;
    let (input, b) = delimited(tag("Register B: "), complete::u64, many1(newline))(input)?;
    let (input, c) = delimited(tag("Register C: "), complete::u64, many1(newline))(input)?;
    let (input, instructions) =
        preceded(tag("Program: "), separated_list1(char(','), complete::u8))(input)?;

    Ok((input, ((b, c), instructions)))
}

fn generate_range(start: u64, stop: u64, n: u64) -> Vec<(u64, u64)> {
    let size = (stop - start) as f64;
    let step_size = (size / n as f64).ceil() as u64;
    (0..n)
        .map(|i| {
            let l = start + i * step_size;
            let r = start + (i + 1) * step_size;

            (l, r.min(stop))
        })
        .collect()
}

pub fn process(input: &str) -> String {
    let (_, ((b, c), instructions_vec)) = parse(input).unwrap();
    let instructions = parse_instructions(&instructions_vec);

    let mut i = 1;
    let mut lower_bound = 0;
    let mut upper_bound = 0;
    let mut found_lower = false;

    loop {
        let mut computer = Computer::new(i, b, c);
        while let Some(instruction) = instructions.get(computer.cur_instruction) {
            instruction.execute(&mut computer);
        }

        if computer.output.len() == instructions_vec.len() && !found_lower {
            lower_bound = i;
            found_lower = true;
        }

        if computer.output.len() > instructions_vec.len() && found_lower {
            upper_bound = i;
            break;
        }

        i *= 2;
    }

    println!(
        "Lower: {}, Upper: {}, Size: {}",
        lower_bound,
        upper_bound,
        upper_bound - lower_bound
    );

    let mut chunks: Vec<(usize, u64, u64)> = generate_range(lower_bound, upper_bound, 100_000_000)
        .par_iter()
        .map(|(start, end)| {
            let mut computer_1 = Computer::new(*start, b, c);
            while let Some(instruction) = instructions.get(computer_1.cur_instruction) {
                instruction.execute(&mut computer_1);
            }

            let valid_1 = computer_1
                .output
                .iter()
                .zip(&instructions_vec)
                .filter(|(a, b)| a == b)
                .count();

            let mut computer_2 = Computer::new(*end, b, c);
            while let Some(instruction) = instructions.get(computer_2.cur_instruction) {
                instruction.execute(&mut computer_2);
            }

            let valid_2 = computer_2
                .output
                .iter()
                .zip(&instructions_vec)
                .filter(|(a, b)| a == b)
                .count();

            (valid_1 + valid_2, *start, *end)
        })
        .collect();

    chunks.sort_by_key(|x| (Reverse(x.0), x.1));

    let mut min_match = u64::MAX;

    for (i, (matches, start, end)) in chunks.into_iter().take(1_000_000).enumerate() {
        println!("CHUNK {}: {}..{} ({} matches)", i, start, end, matches);

        let res = (start..end).into_par_iter().find_first(|a| {
            let mut computer = Computer::new(*a, b, c);
            while let Some(instruction) = instructions.get(computer.cur_instruction) {
                instruction.execute(&mut computer);
            }

            computer.output == instructions_vec
        });

        if let Some(res) = res {
            if min_match > res {
                min_match = res;
                println!("{}", res);
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", process(input));
    }
}
