use crate::utils;
use std::collections::HashMap;

const CHARS: [&'static str; 14] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1",
];

pub struct Solver;

impl utils::Solver<7> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part1_int(input: &str) -> Option<usize> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let s = line.split_once(" ").unwrap();
            Hand::new(s.0, s.1, false)
        })
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    Some(winnings)
}

fn part2_int(input: &str) -> Option<usize> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let s = line.split_once(" ").unwrap();
            Hand::new(s.0, s.1, true)
        })
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    Some(winnings)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum StrengthType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand {
    bid: u16,
    values: (u8, u8, u8, u8, u8),
    strength: StrengthType,
}

impl Hand {
    fn new(cards: &str, bids: &str, part2: bool) -> Self {
        let v: Vec<u8> = cards
            .chars()
            .map(|char| {
                if part2 && char == 'J' {
                    0
                } else {
                    CHARS
                        .iter()
                        .rev()
                        .position(|x| x == &char.to_string())
                        .unwrap() as u8
                }
            })
            .collect();

        Self {
            bid: bids.parse::<u16>().unwrap(),
            values: (v[0], v[1], v[2], v[3], v[4]),
            strength: Self::calc_strength(cards, part2),
        }
    }

    fn calc_strength(cards: &str, part2: bool) -> StrengthType {
        let mut dist: HashMap<char, u16> = HashMap::new();
        for char in cards.chars() {
            dist.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut scores: Vec<u16> = dist
            .keys()
            .map(|key| {
                if part2 && key == &'J' {
                    0_u16
                } else {
                    dist.get(key).unwrap().to_owned()
                }
            })
            .collect();
        scores.sort_by(|a, b| b.cmp(a));

        if part2 {
            scores[0] += cards.chars().filter(|a| a == &'J').count() as u16;
        }

        let strength_type = match scores[0] {
            5 => StrengthType::FiveOfAKind,
            4 => StrengthType::FourOfAKind,
            3 => {
                if scores[1] == 2 {
                    StrengthType::FullHouse
                } else {
                    StrengthType::ThreeOfAKind
                }
            }
            2 => {
                if scores[1] == 2 {
                    StrengthType::TwoPair
                } else {
                    StrengthType::OnePair
                }
            }
            _ => StrengthType::HighCard,
        };

        strength_type
    }
}

#[test]
fn p1_test() {
    let i = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(Some(6440), part1_int(i))
}

#[test]
fn p2_test() {
    let i = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(Some(5905), part2_int(i))
}
