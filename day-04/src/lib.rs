use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, terminated, delimited},
    IResult,
};

#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    chosen: Vec<u32>,
}

fn number(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = separated_list1(multispace1, digit1)(input)?;
    let numbers = numbers
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Ok((input, numbers))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn line(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, id) = terminated(digit1, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, card) = separated_pair(number, delimited(multispace1, tag("|"), multispace1), number)(input)?;
    let card = Card {
        id: id.parse::<u32>().unwrap() - 1,
        winning: card.0,
        chosen: card.1,
    };

    Ok((input, card))
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, games) = separated_list1(line_ending, line)(input)?;
    Ok((input, games))
}

pub fn process_1(input: &str) -> String {
    let (_, games) = cards(input).unwrap();
    games
        .iter()
        .filter_map(|g| {
            let amount = g.chosen.iter().filter(|c| g.winning.contains(c)).count();
            if amount == 0 {
                return None;
            }
            Some((2 as u32).pow(amount as u32 - 1))
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, cards) = cards(input).unwrap();
    let mut no_cards = vec![1; cards.len()];
    cards.iter().for_each(|g| {
        let amount = g.chosen.iter().filter(|c| g.winning.contains(c)).count();
        for _ in 0..no_cards[g.id as usize] {
            for i in (g.id + 1)..(g.id + 1 + amount as u32) {
                no_cards[i as usize] += 1;
            }
        }
    });
    no_cards.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = process_1(input);
        assert_eq!("13", res);
    }

    #[test]
    fn test_process_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = process_2(input);
        assert_eq!("30", res);
    }
}
