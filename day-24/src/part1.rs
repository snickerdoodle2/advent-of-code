use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

type Wires = HashMap<String, u8>;

#[derive(Debug)]
enum GateKind {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Gate {
    kind: GateKind,
    lhs: String,
    rhs: String,
    res: String,
}

impl Gate {
    fn execute(&self, wires: &mut Wires) {
        if wires.get(&self.res).is_some() {
            return;
        }
        let Some(lhs_val) = wires.get(&self.lhs) else {
            return;
        };
        let Some(rhs_val) = wires.get(&self.rhs) else {
            return;
        };

        let res_val = match self.kind {
            GateKind::AND => lhs_val & rhs_val,
            GateKind::OR => lhs_val | rhs_val,
            GateKind::XOR => lhs_val ^ rhs_val,
        };

        wires.insert(self.res.clone(), res_val);
    }
}

fn is_done(to_find: &Vec<String>, wires: &Wires) -> bool {
    for w in to_find {
        if wires.get(w).is_none() {
            return false;
        }
    }
    true
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (input, lhs) = alphanumeric1(input)?;
    let (input, kind) = delimited(
        space1,
        alt((
            tag("AND").map(|_| GateKind::AND),
            tag("OR").map(|_| GateKind::OR),
            tag("XOR").map(|_| GateKind::XOR),
        )),
        space1,
    )(input)?;
    let (input, rhs) = alphanumeric1(input)?;
    let (input, res) = preceded(tag(" -> "), alphanumeric1)(input)?;

    Ok((
        input,
        Gate {
            kind,
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            res: res.to_string(),
        },
    ))
}

fn parse_initial(input: &str) -> IResult<&str, (String, u8)> {
    separated_pair(
        alphanumeric1.map(|x: &str| x.to_string()),
        tag(": "),
        complete::u8,
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Wires, Vec<Gate>)> {
    let (input, initial) = separated_list1(newline, parse_initial)(input)?;
    let (input, gates) = preceded(many1(newline), separated_list1(newline, parse_gate))(input)?;

    Ok((input, (initial.into_iter().collect(), gates)))
}

pub fn process(input: &str) -> String {
    let (_, (mut wires, gates)) = parse(input).unwrap();

    let to_find = {
        let mut to_find: Vec<String> = gates
            .iter()
            .map(|g| g.res.clone())
            .filter(|res| res.starts_with("z"))
            .collect();
        to_find.sort_unstable();
        to_find.reverse();
        to_find
    };

    while !is_done(&to_find, &wires) {
        for g in gates.iter() {
            g.execute(&mut wires)
        }
    }

    let mut res: u64 = 0;

    for z in to_find {
        let val = wires.get(&z).expect("Should be calculated");
        res = (res << 1) | *val as u64;
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        "4"
    )]
    #[case(
        "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        "2024"
    )]
    fn test_part1(#[case] input: &str, #[case] output: String) {
        assert_eq!(output, process(input));
    }
}
