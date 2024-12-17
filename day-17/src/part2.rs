use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, char, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

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
            _ => unreachable!(),
        }
    }
    fn execute(&mut self, instruction: &Instruction) {
        let mut next_instruction: Option<usize> = None;
        match instruction {
            Instruction::Adv(operand) => {
                let denominator: u64 = 2_u64.pow(self.operand(*operand) as u32);
                let res = self.a / denominator;
                self.a = res;
            }
            Instruction::Bxl(operand) => {
                self.b ^= *operand as u64;
            }
            Instruction::Bst(operand) => {
                let operand = self.operand(*operand);
                self.b = operand % 8;
            }
            Instruction::Jnz(operand) => {
                if self.a != 0 {
                    next_instruction = Some(*operand as usize);
                }
            }
            Instruction::Bxc(_) => {
                let res = self.b ^ self.c;
                self.b = res;
            }
            Instruction::Out(operand) => {
                let res = self.operand(*operand) % 8;
                self.output.push(res as u8);
            }
            Instruction::Bdv(operand) => {
                let denominator: u64 = 2_u64.pow(self.operand(*operand) as u32);
                let res = self.a / denominator;
                self.b = res;
            }
            Instruction::Cdv(operand) => {
                let denominator: u64 = 2_u64.pow(self.operand(*operand) as u32);
                let res = self.a / denominator;
                self.c = res;
            }
        }
        self.cur_instruction = next_instruction.unwrap_or_else(|| self.cur_instruction + 1);
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

fn parse(input: &str) -> IResult<&str, (Computer, Vec<Instruction>)> {
    let (input, a) = delimited(tag("Register A: "), complete::u64, many1(newline))(input)?;
    let (input, b) = delimited(tag("Register B: "), complete::u64, many1(newline))(input)?;
    let (input, c) = delimited(tag("Register C: "), complete::u64, many1(newline))(input)?;
    let (input, instructions) =
        preceded(tag("Program: "), separated_list1(char(','), complete::u8))(input)?;

    let instructions = instructions
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut i| {
            let opcode = i.next().unwrap();
            let operand = i.next().unwrap();
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
        .collect();

    Ok((input, (Computer::new(a, b, c), instructions)))
}

pub fn process(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("117440", process(input));
    }
}
