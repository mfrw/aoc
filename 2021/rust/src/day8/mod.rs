use crate::utils;
use std::collections::BTreeSet;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day8/Part1 Sol: {}", part1(&input));
    println!("Day8/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split(" | ");
            let a = iter.next();
            let b = iter.next();
            if a.is_some() && b.is_some() {
                Some((a.unwrap(), b.unwrap()))
            } else {
                None
            }
        })
        .map(|(input, output)| {
            let mut digits = vec![BTreeSet::new(); 10];
            let mut len6: Vec<BTreeSet<u8>> = vec![];
            let mut len5: Vec<BTreeSet<u8>> = vec![];

            for s in input.split_ascii_whitespace() {
                let d = s.bytes().collect();
                match s.len() {
                    2 => digits[1] = d,
                    4 => digits[4] = d,
                    3 => digits[7] = d,
                    7 => digits[8] = d,
                    5 => len5.push(d),
                    6 => len6.push(d),
                    _ => unreachable!(),
                }
            }

            for d in len6 {
                if !d.is_superset(&digits[1]) {
                    digits[6] = d;
                } else if !d.is_superset(&digits[4]) {
                    digits[0] = d;
                } else {
                    digits[9] = d;
                }
            }

            for d in len5 {
                if d.is_subset(&digits[6]) {
                    digits[5] = d;
                } else if d.is_subset(&digits[9]) {
                    digits[3] = d;
                } else {
                    digits[2] = d;
                }
            }

            output.split_ascii_whitespace().fold(0, |acc, s| {
                let signal: BTreeSet<_> = s.bytes().collect();
                let digit = digits.iter().position(|d| *d == signal).unwrap();

                acc * 10 + digit
            })
        })
        .sum()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("|").unwrap();
            let x: Vec<&str> = a.trim().split_whitespace().collect();
            let y: Vec<&str> = b.trim().split_whitespace().collect();
            (x, y)
        })
        .map(|(_, output)| {
            output
                .iter()
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn get_input() -> Result<String, std::io::Error> {
    utils::get_input("input/day8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(294, part1(&input));
    }
    #[test]
    fn part2_test() {
        let input = get_input().unwrap();
        assert_eq!(973292, part2(&input));
    }
}
