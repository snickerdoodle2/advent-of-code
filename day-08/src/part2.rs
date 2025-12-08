use std::collections::HashSet;

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

fn find_box(a: &Box, connections: &[HashSet<Box>]) -> usize {
    connections.iter().position(|con| con.contains(a)).unwrap()
}

pub fn process(input: &str) -> String {
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

    let mut connections: Vec<_> = boxes
        .iter()
        .map(|x| {
            let mut tmp = HashSet::new();
            tmp.insert(*x);
            tmp
        })
        .collect();

    let mut distances = distances.iter();

    let res = loop {
        let (a, b, _) = distances.next().unwrap();
        let a_box = find_box(a, &connections);
        let b_box = find_box(b, &connections);

        if a_box != b_box {
            let (lhs, rhs) = if a_box < b_box {
                (a_box, b_box)
            } else {
                (b_box, a_box)
            };

            let circuit = connections.remove(rhs);
            connections[lhs].extend(circuit);
        }

        if connections.len() == 1 {
            break a.0 * b.0;
        }
    };

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!("25272", process(input));
    }
}
