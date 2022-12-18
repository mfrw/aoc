use crate::utils;
use rayon::prelude::*;

use std::collections::HashSet;

pub struct Solver;

impl utils::Solver<3> for Solver {
    type Part1 = i32;

    type Part2 = i32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let n: i32 = input.lines().map(|l| priority(l)).sum();
        Ok(n)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let v = input.lines().collect::<Vec<_>>();
        let s = v
            .chunks(3)
            .par_bridge()
            .map(|ch| {
                let sta: HashSet<char> = HashSet::from_iter(ch[0].chars());
                let stb = HashSet::from_iter(ch[1].chars());
                let stc = HashSet::from_iter(ch[2].chars());
                let tmp = sta.intersection(&stb).map(|c| *c).collect::<HashSet<_>>();
                let v = tmp.intersection(&stc).collect::<Vec<_>>();
                score(&v)
            })
            .sum();
        Ok(s)
    }
}

fn priority(line: &str) -> i32 {
    let mut l = line.chars().collect::<Vec<char>>();
    let r = l.split_off(l.len() / 2);
    let lst: HashSet<char> = HashSet::from_iter(l);
    let rst: HashSet<char> = HashSet::from_iter(r);
    let v = lst.intersection(&rst).collect::<Vec<_>>();
    let s = score(&v);
    s
}

fn score(v: &[&char]) -> i32 {
    let mut ans = 0;
    for &&c in v {
        if c.is_uppercase() {
            ans += c as i32 - 'A' as i32 + 27;
        } else {
            ans += c as i32 - 'a' as i32 + 1;
        }
    }
    ans
}
