use crate::utils;
use std::{cmp, ops::RangeInclusive};

pub struct Solver;

impl utils::Solver<5> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let a = input.split_once("\n\n").unwrap();
        let mut ranges = vec![];
        for l in a.0.lines() {
            if let Some(r) = l.split_once('-') {
                let begin = r.0.parse::<i64>()?;
                let end = r.1.parse::<i64>()?;
                ranges.push(RangeInclusive::new(begin, end));
            }
        }

        let mut ans = 0;
        for l in a.1.lines() {
            let r = l.parse()?;
            for rs in &ranges {
                if rs.contains(&r) {
                    ans += 1;
                    break;
                }
            }
        }
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let a = input.split_once("\n\n").unwrap();
        let mut ranges = vec![];
        for l in a.0.lines() {
            if let Some(r) = l.split_once('-') {
                let begin = r.0.parse::<i64>()?;
                let end = r.1.parse::<i64>()?;
                ranges.push(RangeInclusive::new(begin, end));
            }
        }
        ranges.sort_by_key(|r| *r.start());
        let mut merged: Vec<RangeInclusive<i64>> = Vec::new();
        merged.push(ranges[0].clone());
        for range in ranges.iter().skip(1) {
            let Some(last) = merged.last_mut() else {
                unreachable!()
            };
            // Check for overlap. Since we want total count of integers,
            // [1, 2] and [3, 4] are contiguous and can be merged for counting purposes?
            // Actually, the problem asks for "how many ingredient IDs".
            // [1, 2] -> 1, 2. [3, 4] -> 3, 4. Total 4.
            // Merged [1, 4] -> 1, 2, 3, 4. Total 4.
            // So yes, we can merge contiguous ranges too.
            if *range.start() <= *last.end() + 1 {
                let new_end = cmp::max(*last.end(), *range.end());
                *last = *last.start()..=new_end;
            } else {
                merged.push(range.clone());
            }
        }

        Ok(merged.iter().map(|r| r.end() - r.start() + 1).sum::<i64>() as usize)
    }
}
