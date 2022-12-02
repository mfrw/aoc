use crate::utils;

pub struct Solver;

impl utils::Solver<1> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input))
    }
}

fn part1_int(input: &str) -> Option<i32> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .max()
}

fn part2_int(input: &str) -> i32 {
    use std::collections::BinaryHeap;
    let hp = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect::<BinaryHeap<i32>>();
    hp.into_iter_sorted().take(3).sum()
}
