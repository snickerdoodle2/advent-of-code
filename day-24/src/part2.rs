use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, space1},
    sequence::delimited,
    IResult, Parser,
};
use petgraph::{
    data::Build,
    dot::Dot,
    graph::{self, NodeIndex, UnGraph},
};

#[derive(Debug)]
enum GateKind {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Gate {
    kind: String,
    lhs: String,
    rhs: String,
    res: String,
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (input, lhs) = alphanumeric1(input)?;
    let (input, kind) = delimited(space1, alt((tag("AND"), tag("OR"), tag("XOR"))), space1)(input)?;
    let (input, rhs) = alphanumeric1(input)?;
    let (input, res) = delimited(tag(" -> "), alphanumeric1, newline)(input)?;

    Ok((
        input,
        Gate {
            kind: kind.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            res: res.to_string(),
        },
    ))
}

pub fn process(input: &str) -> String {
    let mut input = input.split("\n\n").skip(1).next().unwrap();
    let mut graph = UnGraph::<String, String>::new_undirected();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    loop {
        let Ok((
            res_input,
            Gate {
                kind,
                lhs,
                rhs,
                res,
            },
        )) = parse_gate(input)
        else {
            break;
        };
        input = res_input;

        let lhs = *nodes
            .entry(lhs.clone())
            .or_insert_with(|| graph.add_node(lhs));
        let rhs = *nodes
            .entry(rhs.clone())
            .or_insert_with(|| graph.add_node(rhs));
        let res = *nodes
            .entry(res.clone())
            .or_insert_with(|| graph.add_node(res));

        graph.add_edge(lhs, res, kind.clone());
        graph.add_edge(rhs, res, kind);
    }

    Dot::with_config(&graph, &[]).to_string()
}
