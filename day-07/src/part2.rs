use core::panic;
use std::{char, cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardChar {
    T = 0,
    J = 1,
    Q = 2,
    K = 3,
    A = 4,
}

impl CardChar {
    fn new(c: char) -> Self {
        match c {
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Card {
    Digit(u8),
    Letter(CardChar),
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Letter(CardChar::J), Digit(_)) => Ordering::Less,
            (Letter(_), Digit(_)) => Ordering::Greater,
            (Digit(_), Letter(CardChar::J)) => Ordering::Greater,
            (Digit(_), Letter(_)) => Ordering::Less,
            (Letter(CardChar::J), Letter(CardChar::J)) => Ordering::Equal,
            (Letter(CardChar::J), Letter(_)) => Ordering::Less,
            (Letter(_), Letter(CardChar::J)) => Ordering::Greater,
            (Digit(a), Digit(b)) => a.cmp(b),
            (Letter(a), Letter(b)) => a.cmp(b),
        }
    }
}

use Card::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let mut map: HashMap<Card, u8> = HashMap::new();
        let mut jokers: u8 = 0;
        for card in cards.clone() {
            match card {
                Card::Letter(CardChar::J) => {
                    jokers += 1;
                }
                _ => {
                    *map.entry(card).or_default() += 1;
                }
            }
        }

        let mut v = map.values().into_iter().map(|x| *x).collect::<Vec<u8>>();
        v.sort();
        v.reverse();
        if v.len() == 0 {
            v.push(5);
        } else {
            v[0] += jokers;
        }

        let hand_type = match v.get(0) {
            Some(5) => HandType::FiveKind,
            Some(4) => HandType::FourKind,
            Some(3) => match v.get(1) {
                Some(2) => HandType::FullHouse,
                _ => HandType::ThreeKind,
            },
            Some(2) => match v.get(1) {
                Some(2) => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        };

        Self { hand_type, cards }
    }
}

fn parse_cards(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let mut splitted = line.split(" ");
            let cards = splitted.next().unwrap();
            let bid = splitted.next().unwrap().parse::<u32>().unwrap();
            let cards = cards
                .chars()
                .map(|c| {
                    if c.is_digit(10) {
                        Digit(c.to_digit(10).unwrap() as u8)
                    } else {
                        Letter(CardChar::new(c))
                    }
                })
                .collect::<Vec<_>>();
            (Hand::new(cards), bid)
        })
        .collect::<Vec<_>>()
}

pub fn process_2(input: &str) -> String {
    let mut hands = parse_cards(input);
    hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
    dbg!(&hands);
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_hands() {
        let h1 = Hand {
            hand_type: HandType::FiveKind,
            cards: vec![
                Letter(CardChar::J),
                Letter(CardChar::T),
                Letter(CardChar::J),
                Letter(CardChar::T),
                Letter(CardChar::T),
            ],
        };

        let h2 = Hand {
            hand_type: HandType::FiveKind,
            cards: vec![
                Letter(CardChar::J),
                Letter(CardChar::J),
                Letter(CardChar::Q),
                Letter(CardChar::Q),
                Letter(CardChar::Q),
            ],
        };

        assert_eq!(h1.cmp(&h2), Ordering::Greater)
    }

    #[test]
    fn test_sorting_2() {
        let mut v = (2..=9)
            .map(|x| Digit(x))
            .chain(
                ['A', 'K', 'Q', 'J', 'T']
                    .iter()
                    .map(|x| Letter(CardChar::new(*x))),
            )
            .collect::<Vec<_>>();
        let res = vec![
            Letter(CardChar::A),
            Letter(CardChar::K),
            Letter(CardChar::Q),
            Letter(CardChar::T),
            Digit(9),
            Digit(8),
            Digit(7),
            Digit(6),
            Digit(5),
            Digit(4),
            Digit(3),
            Digit(2),
            Letter(CardChar::J),
        ];
        v.sort();
        v.reverse();

        assert_eq!(res, v);
    }

    #[test]
    fn test_process_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let res = process_2(input);
        assert_eq!("5905", res);
    }
}
