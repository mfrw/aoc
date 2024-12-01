use crate::utils;
use std::collections::HashMap;

pub struct Solver;

impl utils::Solver<1> for Solver {
    type Part1 = i32;

    type Part2 = i32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part2_int(input: &str) -> Option<i32> {
    let mut n1 = vec![];
    let mut mp2: HashMap<i32, i32> = HashMap::new();
    for l in input.lines() {
        let v = l
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        n1.push(v[0]);
        mp2.entry(v[1]).and_modify(|e| *e += 1).or_insert(1);
    }
    let ans = n1
        .iter()
        .map(|v| {
            if let Some(freq) = mp2.get(v) {
                freq * v
            } else {
                0
            }
        })
        .sum();
    Some(ans)
}

fn part1_int(input: &str) -> Option<i32> {
    let mut n1 = vec![];
    let mut n2 = vec![];
    for l in input.lines() {
        let v = l
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        n1.push(v[0]);
        n2.push(v[1]);
    }
    n1.sort_unstable();
    n2.sort_unstable();
    Some(n1.iter().zip(n2).map(|(v1, v2)| (v1 - v2).abs()).sum())
}
