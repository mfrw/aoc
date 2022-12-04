use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::utils;

pub struct Solver;

impl utils::Solver<4> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let sol = input
            .lines()
            .map(|l| {
                let mut rs = l.split(",");
                let e1 = rs.next().unwrap().parse::<ElfRange>().unwrap();
                let e2 = rs.next().unwrap().parse::<ElfRange>().unwrap();
                if e1.fully_contains(&e2) || e2.fully_contains(&e1) {
                    1
                } else {
                    0
                }
            })
            .sum();
        Ok(sol)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let sol = input
            .lines()
            .map(|l| {
                let mut rs = l.split(",");
                let e1 = rs.next().unwrap().parse::<ElfRange>().unwrap();
                let e2 = rs.next().unwrap().parse::<ElfRange>().unwrap();
                if e1.overlap(&e2) || e2.overlap(&e1) {
                    1
                } else {
                    0
                }
            })
            .sum();
        Ok(sol)
    }
}

struct ElfRange {
    from: i32,
    to: i32,
}

impl ElfRange {
    fn fully_contains(&self, other: &ElfRange) -> bool {
        let rs = RangeInclusive::new(self.from, self.to);
        rs.contains(&other.from) && rs.contains(&other.to)
    }

    fn overlap(&self, other: &ElfRange) -> bool {
        let rs = RangeInclusive::new(self.from, self.to);
        rs.contains(&other.from) || rs.contains(&other.to)
    }
}

impl FromStr for ElfRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("-");
        let f = it.next().unwrap().parse::<i32>()?;
        let t = it.next().unwrap().parse::<i32>()?;
        Ok(Self { from: f, to: t })
    }
}
