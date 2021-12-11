use crate::utils::{self, Result};
use std::collections::HashMap;

pub fn main() -> Result<()> {
    let input = get_input()?;
    println!("Day2/Part1 : {}", part1(&input));
    println!("Day2/Part2 : {}", part2(&input));

    Ok(())
}

fn part2(input: &[(usize, usize, char, String)]) -> usize {
    input
        .iter()
        .filter(|e| {
            let pos1 = e.0;
            let pos2 = e.1;
            let mut count = 0;
            e.3.chars().enumerate().for_each(|(idx, ch)| {
                if (pos1 == idx || pos2 == idx) && ch == e.2 {
                    count += 1;
                }
            });
            if count == 1 {
                true
            } else {
                false
            }
        })
        .count()
}

fn part1(input: &[(usize, usize, char, String)]) -> usize {
    input
        .iter()
        .filter(|e| {
            let mut mp = HashMap::new();
            (e.3).chars().for_each(|ch| {
                mp.entry(ch).and_modify(|e| *e += 1).or_insert(1);
            });
            let low = e.0;
            let hi = e.1;
            let ch = e.2;
            let val = mp.get(&ch);

            if (val.is_some()) && (val.unwrap() >= &low && val.unwrap() <= &hi) {
                true
            } else {
                false
            }
        })
        .count()
}

fn get_input() -> Result<Vec<(usize, usize, char, String)>> {
    let input = utils::get_input("input/day2")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<(usize, usize, char, String)>> {
    let v = input
        .lines()
        .map(|l| {
            let (policy, password) = l.split_once(":").unwrap();
            let (range, ch) = policy.split_once(" ").unwrap();
            let (low, high) = range.split_once("-").unwrap();
            (
                low.parse::<usize>().unwrap(),
                high.parse::<usize>().unwrap(),
                ch.chars().next().unwrap(),
                password.to_string(),
            )
        })
        .collect::<Vec<_>>();
    Ok(v)
}
