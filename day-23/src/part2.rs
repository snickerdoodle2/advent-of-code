use std::collections::{HashMap, HashSet};

use petgraph::graph::{NodeIndex, UnGraph};

type LanParty = UnGraph<(), ()>;
type Nodes = Vec<NodeIndex>;

fn bron_kerbosch(r: Nodes, mut p: Nodes, mut x: Nodes, graph: &LanParty) -> Nodes {
    if p.len() == 0 && x.len() == 0 {
        return r;
    }
    let mut res = vec![];

    for v in p.clone().into_iter() {
        let neighbors = graph.neighbors(v).collect::<Vec<_>>();

        let mut nr = r.clone();
        nr.push(v);

        let np = p
            .clone()
            .into_iter()
            .filter(|c| neighbors.contains(c))
            .collect();

        let nx = x
            .clone()
            .into_iter()
            .filter(|c| neighbors.contains(c))
            .collect();

        let cur = bron_kerbosch(nr, np, nx, graph);
        if cur.len() > res.len() {
            res = cur;
        }
        p = p.into_iter().filter(|c| *c != v).collect();
        x.push(v);
    }

    res
}

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

    let p = nodes.values().cloned().collect();

    let res = bron_kerbosch(vec![], p, vec![], &graph);

    let node_names: HashMap<NodeIndex, String> = nodes.into_iter().map(|(k, v)| (v, k)).collect();

    let mut res: Vec<_> = res
        .iter()
        .map(|c| node_names.get(c).unwrap().clone())
        .collect();
    res.sort_unstable();

    res.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";
        assert_eq!("co,de,ka,ta", process(input));
    }
}
