use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl Display for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

fn parse_spring_1(input: &str) -> IResult<&str, Vec<SpringState>> {
    many1(alt((
        tag(".").map(|_| SpringState::Operational),
        tag("#").map(|_| SpringState::Damaged),
        tag("?").map(|_| SpringState::Unknown),
    )))(input)
}

fn parse_groups_1(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), complete::u64)(input)
}

fn parse_spring_2(input: &str) -> IResult<&str, Vec<SpringState>> {
    let (input, mut springs) = many1(alt((
        tag(".").map(|_| SpringState::Operational),
        tag("#").map(|_| SpringState::Damaged),
        tag("?").map(|_| SpringState::Unknown),
    )))(input)?;

    let springs_cp = springs.clone();

    (0..4).for_each(|_| {
        springs.push(SpringState::Unknown);
        for ele in springs_cp.iter() {
            springs.push(ele.clone());
        }
    });
    Ok((input, springs))
}

fn parse_groups_2(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, mut groups) = separated_list1(tag(","), complete::u64)(input)?;

    let groups_cp = groups.clone();

    (0..4).for_each(|_| {
        for ele in groups_cp.iter() {
            groups.push(ele.clone());
        }
    });
    Ok((input, groups))
}

fn parse_row_1(input: &str) -> IResult<&str, (Vec<SpringState>, Vec<u64>)> {
    separated_pair(parse_spring_1, multispace1, parse_groups_1)(input)
}

fn parse_row_2(input: &str) -> IResult<&str, (Vec<SpringState>, Vec<u64>)> {
    separated_pair(parse_spring_2, multispace1, parse_groups_2)(input)
}

fn solve(springs: Vec<SpringState>, groups: Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    let key = format!(
        "{}{}",
        springs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(""),
        groups
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let value = calc(springs.clone(), groups.clone(), cache);
    cache.insert(key, value);
    value
}

fn calc(springs: Vec<SpringState>, groups: Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    let mut springs = springs;
    let mut groups = groups;

    loop {
        // no groups left
        if groups.len() == 0 {
            return if springs
                .iter()
                .filter(|x| x == &&SpringState::Damaged)
                .count()
                > 0
            {
                0
            } else {
                1
            };
        }

        // no springs but still a group
        if springs.len() == 0 {
            return 0;
        }

        match springs.first().unwrap() {
            // remove all operational from beginning
            SpringState::Operational => {
                springs.remove(0);
                continue;
            }
            // run two instances
            SpringState::Unknown => {
                let mut left = springs.clone();
                let mut right = springs.clone();
                *left.get_mut(0).unwrap() = SpringState::Operational;
                *right.get_mut(0).unwrap() = SpringState::Damaged;
                return solve(left, groups.clone(), cache) + solve(right, groups.clone(), cache);
            }
            SpringState::Damaged => {
                // if there are no enough springs to satisfy group
                if springs.len() < groups[0] as usize {
                    return 0;
                }

                // if there is an operational spring inside group
                if (0..groups[0])
                    .map(|x| springs[x as usize])
                    .filter(|x| x == &SpringState::Operational)
                    .count()
                    > 0
                {
                    return 0;
                }

                // if there is more than one group
                if groups.len() > 1 {
                    // if there is damaged spring after group
                    if springs.len() < groups[0] as usize + 1
                        || springs[groups[0] as usize] == SpringState::Damaged
                    {
                        return 0;
                    }

                    (0..=groups[0]).rev().for_each(|i| {
                        springs.remove(i as usize);
                    });

                    groups.remove(0);
                    continue;
                }

                (0..groups[0]).rev().for_each(|i| {
                    springs.remove(i as usize);
                });

                groups.remove(0);
            }
        }
    }
}

pub fn process_1(input: &str) -> String {
    let (_, rows) = separated_list1(newline, parse_row_1)(input).unwrap();
    let mut cache: HashMap<String, u64> = HashMap::new();

    rows.into_iter()
        .map(|row| solve(row.0, row.1, &mut cache))
        .sum::<u64>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, rows) = separated_list1(newline, parse_row_2)(input).unwrap();
    let mut cache: HashMap<String, u64> = HashMap::new();

    rows.into_iter()
        .map(|row| solve(row.0, row.1, &mut cache))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1_1() {
        let input = "???.### 1,1,3";
        let res = process_1(input);
        assert_eq!("1", res);
    }

    #[test]
    fn test_process_1_2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let res = process_1(input);
        assert_eq!("21", res);
    }

    #[test]
    fn test_process_2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let res = process_2(input);
        assert_eq!("525152", res);
    }
}
