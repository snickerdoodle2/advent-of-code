use std::collections::HashMap;

use nom::{
    character::complete::{self, char, newline},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

type Rules = HashMap<u16, Vec<u16>>;
#[derive(Debug)]
struct Update(Vec<u16>);

impl Update {
    fn check_validity(&self, rules: &Rules) -> bool {
        for (i, x) in self.0.iter().enumerate() {
            for j in (i + 1)..self.0.len() {
                let Some(r) = rules.get(&self.0[j]) else {
                    continue;
                };
                if r.contains(x) {
                    return false;
                }
            }
        }

        true
    }

    fn fixup(&mut self, rules: &Rules) {
        let mut deps: Vec<_> = self
            .0
            .iter()
            .map(|cur| {
                (
                    *cur,
                    rules
                        .get(cur)
                        .cloned()
                        .map(|v| {
                            v.into_iter()
                                .filter(|other| self.0.contains(other))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default(),
                )
            })
            .collect();

        deps.sort_by_key(|e| e.1.len());

        self.0 = deps.iter().rev().map(|x| x.0).collect();
    }

    fn get_value(&self, rules: &Rules) -> Option<u64> {
        assert_eq!(self.0.len() % 2, 1);
        if self.check_validity(rules) {
            let index = self.0.len() as u64 / 2;
            self.0.get(index as usize).map(|x| *x as u64)
        } else {
            None
        }
    }
}

fn parse_rules(input: &str) -> IResult<&str, Rules> {
    let (input, rules) = many1(terminated(
        separated_pair(complete::u16, char('|'), complete::u16),
        newline,
    ))(input)?;

    let rules = rules.into_iter().fold(Rules::new(), |mut acc, (k, v)| {
        acc.entry(k).or_default().push(v);
        acc
    });

    Ok((input, rules))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list1(
        newline,
        separated_list1(char(','), complete::u16).map(|x| Update(x)),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Rules, Vec<Update>)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = newline(input)?;
    let (input, updates) = parse_updates(input)?;

    Ok((input, (rules, updates)))
}

pub fn process(input: &str) -> String {
    let (_, (rules, mut updates)) = parse(input).unwrap();

    let mut res: Vec<_> = updates
        .iter_mut()
        .filter(|x| !x.check_validity(&rules))
        .collect();

    res.iter_mut().for_each(|x| x.fixup(&rules));

    res.iter()
        .filter_map(|x| x.get_value(&rules))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input));
    }
}
