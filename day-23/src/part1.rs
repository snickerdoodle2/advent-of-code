use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};

type LanParty = UnGraph<(), ()>;

fn create_graph(
    graph: &mut LanParty,
    nodes: &mut HashMap<String, NodeIndex>,
    lhs: String,
    rhs: String,
) {
    let lhs = {
        if let Some(lhs) = nodes.get(&lhs) {
            *lhs
        } else {
            let idx = graph.add_node(());
            nodes.insert(lhs, idx);
            idx
        }
    };

    let rhs = {
        if let Some(rhs) = nodes.get(&rhs) {
            *rhs
        } else {
            let idx = graph.add_node(());
            nodes.insert(rhs, idx);
            idx
        }
    };

    graph.add_edge(lhs, rhs, ());
}

type Parties = HashSet<[NodeIndex; 3]>;

fn find_parties(starting: NodeIndex, graph: &LanParty) -> Parties {
    let neighbors = graph.neighbors(starting);
    let mut res = Parties::new();
    for (lhs, rhs) in neighbors.tuple_combinations() {
        if graph.contains_edge(lhs, rhs) {
            let key = {
                let mut tmp = vec![starting, lhs, rhs];
                tmp.sort();
                (tmp[0], tmp[1], tmp[2])
            };
            res.insert(key.into());
        }
    }

    res
}

pub fn process(input: &str) -> String {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::new();
    let mut potential = HashSet::new();

    for line in input.lines() {
        let mut line = line.split("-");
        let lhs = line.next().unwrap().to_string();
        let rhs = line.next().unwrap().to_string();
        if lhs.starts_with("t") {
            potential.insert(lhs.clone());
        }
        if rhs.starts_with("t") {
            potential.insert(rhs.clone());
        }

        create_graph(&mut graph, &mut nodes, lhs, rhs);
    }

    potential
        .iter()
        .flat_map(|name| {
            let node = nodes.get(name).unwrap();

            find_parties(*node, &graph)
        })
        .collect::<Parties>()
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("7", process(input));
    }
}
