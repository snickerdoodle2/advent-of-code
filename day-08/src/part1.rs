use std::{cmp, collections::HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Box(u64, u64, u64);

fn parse(input: &str) -> IResult<&str, Vec<Box>> {
    separated_list1(
        newline,
        separated_list1(tag(","), complete::u64).map(|b| Box(b[0], b[1], b[2])),
    )(input)
}

fn distance(a: &Box, b: &Box) -> u64 {
    (a.0 as i64 - b.0 as i64).pow(2) as u64
        + (a.1 as i64 - b.1 as i64).pow(2) as u64
        + (a.2 as i64 - b.2 as i64).pow(2) as u64
}

fn find_box(a: &Box, connections: &[HashSet<Box>]) -> Option<usize> {
    connections.iter().position(|con| con.contains(a))
}

pub fn process(input: &str, count: usize) -> String {
    let (_, boxes) = parse(input).unwrap();

    let mut distances = vec![];

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let a = &boxes[i];
            let b = &boxes[j];
            distances.push((a, b, distance(a, b)));
        }
    }

    distances.sort_by_key(|a| a.2);

    let mut connections = vec![];

    for (a, b, _) in &distances[..count] {
        let box_a = find_box(a, &connections);
        let box_b = find_box(b, &connections);
        match (box_a, box_b) {
            (Some(idx), None) | (None, Some(idx)) => {
                connections[idx].insert(**a);
                connections[idx].insert(**b);
            }
            (Some(a_idx), Some(b_idx)) => {
                if a_idx != b_idx {
                    let (lhs, rhs) = if a_idx < b_idx {
                        (a_idx, b_idx)
                    } else {
                        (b_idx, a_idx)
                    };

                    let circuit = connections.remove(rhs);
                    connections[lhs].extend(circuit);
                }
            }
            (None, None) => {
                let mut tmp = HashSet::new();
                tmp.insert(**a);
                tmp.insert(**b);
                connections.push(tmp);
            }
        }
    }

    let mut cons: Vec<_> = connections.iter().map(|c| c.len()).collect();
    cons.sort_by_key(|x| cmp::Reverse(*x));

    cons.iter().take(3).product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
        assert_eq!("40", process(input, 10));
    }
}
