use std::{
    collections::{HashMap, VecDeque},
    usize,
};

#[derive(Debug)]
struct Tree([Option<Box<Node>>; 26]);

#[derive(Debug)]
struct Node {
    valid: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn new() -> Self {
        Self {
            valid: false,
            children: [const { None }; 26],
        }
    }
}

#[inline]
fn char_to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn add_to_tree(tree: &mut Tree, cur: &str) {
    let n = cur.len();
    let mut cur_node = &mut tree.0;

    for (i, c) in cur.chars().enumerate() {
        if cur_node[char_to_index(c)].is_none() {
            cur_node[char_to_index(c)] = Some(Box::new(Node::new()));
        }

        if i == n - 1 {
            let node = &mut cur_node[char_to_index(c)].as_mut().unwrap().as_mut();
            node.valid = true;
        }
        cur_node = &mut cur_node[char_to_index(c)]
            .as_mut()
            .unwrap()
            .as_mut()
            .children;
    }
}

type Cache<'a> = HashMap<String, usize>;

fn check_combination(pattern: &str, tree: &Tree, cache: &mut Cache) -> usize {
    if pattern.len() == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(pattern) {
        return *cached;
    }

    let mut cur_node = &tree.0;
    let mut stack = VecDeque::new();
    for (i, c) in pattern.chars().enumerate() {
        if let Some(node) = &cur_node[char_to_index(c)] {
            if node.valid {
                stack.push_back(i + 1);
            }
            cur_node = &node.children;
        } else {
            break;
        }
    }

    let mut result = 0;

    while let Some(i) = stack.pop_back() {
        result += check_combination(&pattern[i..], tree, cache);
    }

    cache.insert(pattern.to_string(), result);

    result
}

pub fn process(input: &str) -> String {
    let mut lines = input.lines();
    let mut tree = Tree([const { None }; 26]);

    for available in lines.next().unwrap().split(", ") {
        add_to_tree(&mut tree, available);
    }

    let res: usize = lines
        .skip(1)
        .map(|pattern| {
            let mut cache = Cache::new();
            let res = check_combination(pattern, &tree, &mut cache);
            res
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "wr, b, g, bwu, rb, gb, br, r

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("16", process(input));
    }
}
