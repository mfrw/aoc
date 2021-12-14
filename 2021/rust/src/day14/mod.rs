use std::collections::HashMap;

use crate::utils::{self, stack::Stack};

pub fn main() -> Result<(), std::io::Error> {
    let (start, mp) = get_input()?;
    println!("Day14/Part1 Sol: {}", part1(&start, &mp, 10));
    println!("Day14/Part2 Sol: {}", part2(&start, &mp, 40));
    Ok(())
}

fn part2(start: &str, rules: &HashMap<(char, char), char>, nr: usize) -> usize {
    let mut counts = HashMap::new();
    for citer in start.chars().collect::<Vec<_>>().windows(2) {
        let (a, b): (char, char) = (citer[0], citer[1]);
        counts.entry((a, b)).and_modify(|v| *v += 1).or_insert(1);
    }
    for _ in 0..nr {
        let mut curr = HashMap::<(char, char), usize>::new();
        for (k, c) in counts.iter() {
            if let Some(ins) = rules.get(&k) {
                curr.entry((k.0, *ins))
                    .and_modify(|v| *v += c)
                    .or_insert(*c);
                curr.entry((*ins, k.1))
                    .and_modify(|v| *v += c)
                    .or_insert(*c);
            }
        }
        counts = curr;
    }
    let mut counts: HashMap<char, usize> = counts.iter().fold(HashMap::new(), |mut m, (k, v)| {
        m.entry(k.1).and_modify(|e| *e += v).or_insert(*v);
        m
    });

    let first = start.chars().next().unwrap();
    counts.entry(first).and_modify(|v| *v += 1).or_insert(1);
    let max = counts.iter().max_by_key(|(_, i)| *i).unwrap().1;
    let min = counts.iter().min_by_key(|(_, i)| *i).unwrap().1;
    max - min
}

fn part1(start: &str, rules: &HashMap<(char, char), char>, nr: usize) -> usize {
    let mut str = String::from(start);
    for _ in 0..nr {
        str = apply_once(&str, rules);
    }

    let mut mp = HashMap::new();
    str.chars().for_each(|ch| {
        mp.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    });
    let min = *mp.values().min().unwrap();
    let max = *mp.values().max().unwrap();
    max - min
}

fn apply_once(start: &str, rules: &HashMap<(char, char), char>) -> String {
    let mut stk = Stack::<char>::new();

    for ch in start.chars() {
        if stk.is_empty() {
            stk.push(ch);
            continue;
        }

        let prev = *stk.top().unwrap();
        if let Some(ins) = rules.get(&(prev, ch)) {
            stk.push(*ins);
        }
        stk.push(ch);
    }
    // all chars are in
    // now convert back to a string
    let mut result = String::new();
    for ch in stk {
        result.push(ch);
    }
    result.chars().rev().collect()
}

fn get_input() -> Result<(String, HashMap<(char, char), char>), std::io::Error> {
    let input = utils::get_input("input/day14")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<(String, HashMap<(char, char), char>), std::io::Error> {
    let (start, rules) = input.split_once("\n\n").unwrap();
    let mut mp = HashMap::new();

    rules
        .lines()
        .map(|l| {
            let (mat, insert) = l.split_once(" -> ").unwrap();
            let mut matiter = mat.chars();
            let (a, b): (char, char) = (matiter.next().unwrap(), matiter.next().unwrap());
            let ins: char = insert.chars().next().unwrap();
            ((a, b), ins)
        })
        .for_each(|rule| {
            mp.entry(rule.0).or_insert(rule.1);
        });
    Ok((start.to_string(), mp))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_custom_input() -> (String, HashMap<(char, char), char>) {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        parse_input(input).unwrap()
    }

    #[test]
    fn part1_custom_test() {
        let (start, mp) = get_custom_input();
        let sol = part1(&start, &mp, 5);
        assert_eq!(sol, 33);
    }
    #[test]
    fn part2_custom_test() {
        let (start, mp) = get_custom_input();
        let sol = part2(&start, &mp, 50);
        assert_eq!(sol, 2248783425710274);
    }
    #[test]
    fn part1_test() {
        let (start, mp) = get_input().unwrap();
        let sol = part1(&start, &mp, 10);
        assert_eq!(sol, 4244);
    }
    #[test]
    fn part2_test() {
        let (start, mp) = get_input().unwrap();
        let sol = part2(&start, &mp, 40);
        assert_eq!(sol, 4807056953866);
    }
}
