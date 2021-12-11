use crate::utils::{self, Result};
use std::collections::HashSet;

pub fn main() -> Result<()> {
    let input = get_input()?;
    println!("Day1/Part1 : {}", part1(&input, 2020).unwrap());
    println!("Day1/Part2 : {}", part2(&input, 2020).unwrap());
    Ok(())
}

fn two_sum(input: &[i32], target: i32) -> Option<i32> {
    let mut st = HashSet::new();
    for i in input {
        if st.contains(i) {
            return Some(i * (target - i));
        }
        st.insert(target - i);
    }
    None
}

fn part2(input: &[i32], target: i32) -> Option<i32> {
    for (idx, &n) in input.into_iter().enumerate() {
        let f = n;
        if let Some(lower) = two_sum(&[&input[..idx], &input[idx + 1..]].concat(), target - f) {
            return Some(lower * f);
        };
    }
    None
}

fn part1(input: &[i32], target: i32) -> Option<i32> {
    two_sum(input, target)
}

fn get_input() -> Result<Vec<i32>> {
    let input = utils::get_input("input/day1")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<i32>> {
    let v = input
        .lines()
        .map(str::parse::<i32>)
        .map(|nr| -> Result<i32> {
            let nr = nr?;
            Ok(nr)
        })
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    Ok(v)
}
