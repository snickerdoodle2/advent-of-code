use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
enum Operation {
    Sub,
    Add(u32),
}

#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> Step<'a> {
    fn get_hash(&self) -> u32 {
        let mut res = 0;
        self.label.chars().for_each(|c| {
            let ascii = c as u8;
            res += ascii as u32;
            res = (res * 17) % 256;
        });
        res
    }
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, label) = alpha1(input)?;
    let (input, operation) = alt((
        tag("-").map(|_| Operation::Sub),
        preceded(tag("="), complete::u32).map(|x| Operation::Add(x)),
    ))(input)?;
    Ok((input, Step { label, operation }))
}

pub fn process_1(input: &str) -> String {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split(',')
        .map(|word| {
            let mut res = 0;
            word.chars().for_each(|c| {
                let ascii = c as u8;
                res += ascii as u32;
                res = (res * 17) % 256;
            });
            res
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, steps) = separated_list1(tag(","), parse_step)(input).unwrap();
    let mut boxes: HashMap<u32, VecDeque<(&str, u32)>> = HashMap::new();
    steps.iter().for_each(|s| {
        let box_ = boxes.entry(s.get_hash()).or_insert(VecDeque::new());
        match s.operation {
            Operation::Add(v) => {
                if let Some(index) = box_.iter().position(|x| x.0 == s.label) {
                    *box_.get_mut(index).unwrap() = (s.label, v);
                } else {
                    box_.push_back((s.label, v));
                }
            }
            Operation::Sub => {
                if let Some(index) = box_.iter().position(|x| x.0 == s.label) {
                    box_.remove(index);
                }
            }
        }
    });

    boxes
        .iter()
        .map(|(box_id, v)| {
            v.iter()
                .enumerate()
                .map(|(i, (_, x))| (*box_id as usize + 1) * (i + 1) * *x as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let res = process_1(input);
        assert_eq!("1320", res);
    }

    #[test]
    fn test_process_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let res = process_2(input);
        assert_eq!("145", res);
    }
}
