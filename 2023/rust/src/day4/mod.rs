use crate::utils;
use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace1,
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub struct Solver;

impl utils::Solver<4> for Solver {
    type Part1 = u32;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part1_int(input: &str) -> Option<u32> {
    let mut ans = 0;
    for line in input.lines() {
        match parse_card(line) {
            Ok((_, c)) => {
                //println!("{:?} - {:?} - {:?} ", c.id, c.got, c.win);
                let st1 = HashSet::<usize>::from_iter(c.got);
                let st2 = HashSet::<usize>::from_iter(c.win);
                let cnt = st1.intersection(&st2).count();
                if cnt == 0 {
                    continue;
                }
                ans += 2u32.pow((cnt - 1) as u32);
            }
            Err(e) => println!("{e}"),
        }
    }
    Some(ans)
}

fn win_cards(cards: &[Card], copies_count: &mut [usize], card_number: usize) {
    let st1 = HashSet::<&usize>::from_iter(cards[card_number - 1].got.iter());
    let st2 = HashSet::<&usize>::from_iter(cards[card_number - 1].win.iter());
    let cnt = st1.intersection(&st2).count();

    for next_card_number in card_number + 1..=card_number + cnt {
        copies_count[next_card_number - 1] += copies_count[card_number - 1];
    }
}

fn part2_int(input: &str) -> Option<usize> {
    let mut cards = vec![];
    for line in input.lines() {
        cards.push(parse_card(line).unwrap().1);
    }
    let mut copies_count = vec![1; cards.len()];

    for card_number in 1..copies_count.len() {
        win_cards(&cards, &mut copies_count, card_number);
    }
    Some(copies_count.iter().sum())
}

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    id: usize,
    got: Vec<usize>,
    win: Vec<usize>,
}

fn parse_card(i: &str) -> IResult<&str, Card> {
    let (i, (_, id, got, win)) = tuple((
        terminated(tag("Card"), multispace1),
        terminated(parse_number, terminated(tag(":"), multispace1)),
        parse_number_list,
        preceded(
            delimited(multispace1, tag("|"), multispace1),
            parse_number_list,
        ),
    ))(i)?;
    Ok((i, Card { id, got, win }))
}

fn parse_number_list(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(multispace1, parse_number)(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

#[test]
fn p1_test() {
    let i = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(Some(13), part1_int(i))
}

#[test]
fn p2_test() {
    let i = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(Some(30), part2_int(i))
}
