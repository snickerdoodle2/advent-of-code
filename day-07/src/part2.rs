use itertools::repeat_n;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    Combination,
}

const OPS: [Operation; 3] = [
    Operation::Addition,
    Operation::Multiplication,
    Operation::Combination,
];

fn generate_operations(n: usize) -> impl Iterator<Item = Vec<&'static Operation>> {
    repeat_n(OPS.iter(), n).multi_cartesian_product()
}

#[derive(Debug)]
struct Equation {
    answer: u64,
    parts: Vec<u64>,
}

impl Equation {
    fn can_be_valid(&self) -> bool {
        let mut opts = generate_operations(self.parts.len() - 1);
        opts.any(|ops| {
            let mut remaining_parts = self.parts.iter();
            let mut res = *remaining_parts.next().unwrap();
            for (num, op) in remaining_parts.zip(ops) {
                if res > self.answer {
                    return false;
                }
                match op {
                    Operation::Addition => res += num,
                    Operation::Multiplication => res *= num,
                    Operation::Combination => {
                        let shift = (*num as f32).log10() as u32 + 1;
                        res = res * (10 as u64).pow(shift) + num;
                    }
                }
            }

            res == self.answer
        })
    }
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, answer) = complete::u64(input)?;
    let (input, parts) = preceded(tag(": "), separated_list1(tag(" "), complete::u64))(input)?;

    Ok((input, Equation { answer, parts }))
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(newline, parse_equation)(input)
}

pub fn process(input: &str) -> String {
    let (_, equations) = parse(input).unwrap();

    equations
        .par_iter()
        .filter_map(|eq| {
            if eq.can_be_valid() {
                Some(eq.answer)
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
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!("11387", process(input));
    }
}
