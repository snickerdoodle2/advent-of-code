use std::collections::BTreeMap;

use nom::{
    branch::alt,
    character::complete::{anychar, digit1},
    multi::many1,
    IResult,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Symbol {
    Dot,
    Number(u32),
    Symbol(char),
}

fn number(input: &str) -> IResult<&str, Symbol> {
    let (input, n) = digit1(input)?;
    Ok((input, Symbol::Number(n.parse().unwrap())))
}

fn other_symbol(input: &str) -> IResult<&str, Symbol> {
    let (input, c) = anychar(input)?;
    match c {
        '.' => Ok((input, Symbol::Dot)),
        _ => Ok((input, Symbol::Symbol(c))),
    }
}

fn line_parse(input: &str) -> IResult<&str, Vec<Symbol>> {
    let (_, symbols) = many1(alt((number, other_symbol)))(input)?;
    Ok((input, symbols))
}

pub fn process_1(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| {
            let (_, symbols) = line_parse(line).unwrap();
            symbols
        })
        .collect_vec();

    let mut map: BTreeMap<(usize, usize), Symbol> = BTreeMap::new();
    for (y, line) in lines.iter().enumerate() {
        let mut padding = 0;
        for (x, symbol) in line.iter().enumerate() {
            map.insert((x + padding, y), symbol.clone());
            if let Symbol::Number(n) = symbol {
                padding += n.to_string().len() - 1;
            }
        }
    }

    map.clone()
        .into_iter()
        .filter_map(|((x, y), symbol)| match symbol {
            Symbol::Number(n) => {
                let size = n.to_string().len();
                let x_range = ((x as i32 - 1).max(0) as usize)..=(x + size);
                let y_range = ((y as i32 - 1).max(0) as usize)..=(y + 1);
                for (x_new, y_new) in x_range.cartesian_product(y_range) {
                    if let Some(Symbol::Symbol(_)) = map.get(&(x_new, y_new)) {
                        return Some(n);
                    }
                }

                None
            }
            _ => None,
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| {
            let (_, symbols) = line_parse(line).unwrap();
            symbols
        })
        .collect_vec();

    let mut map: BTreeMap<(usize, usize), Symbol> = BTreeMap::new();
    for (y, line) in lines.iter().enumerate() {
        let mut padding = 0;
        for (x, symbol) in line.iter().enumerate() {
            map.insert((x + padding, y), symbol.clone());
            if let Symbol::Number(n) = symbol {
                padding += n.to_string().len() - 1;
            }
        }
    }

    map.clone()
        .into_iter()
        .filter_map(|((x, y), symbol)| match symbol {
            Symbol::Symbol('*') => {
                let mut parts_nearby = vec![];

                let x_range = (x - 1)..=(x + 1);
                for new_y in ((y as i32 - 1).max(0) as usize)..=((y + 1).min(lines.len() - 1)) {
                    let mut padding = 0;
                    lines
                        .get(new_y)
                        .unwrap()
                        .into_iter()
                        .enumerate()
                        .for_each(|(new_x, s)| {
                            match s {
                                Symbol::Number(n) => {
                                    let size = n.to_string().len();
                                    if ((new_x + padding)..=(new_x + padding + size - 1)).any(|x| x_range.contains(&x)) {
                                        parts_nearby.push(n);
                                    }
                                    padding += size - 1;
                                },
                                _ => ()
                            }
                        });
                }

                if parts_nearby.len() == 2 {
                    return Some(parts_nearby[0] * parts_nearby[1]);
                }

                None
            }
            _ => None,
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_one() {
        assert_eq!(process_1(INPUT), "4361");
    }

    #[test]
    fn test_two() {
        assert_eq!(process_2(INPUT), "467835");
    }
}
