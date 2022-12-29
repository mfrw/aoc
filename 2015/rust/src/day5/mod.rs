use itertools::Itertools;
use std::str;

use crate::utils;

pub struct Day;

impl utils::Solver<5> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let ans = i.lines().filter(|&l| apply_rules(l)).count();
        Ok(ans)
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let ans = i.lines().filter(|&l| apply_rules2(l)).count();
        Ok(ans)
    }
}

fn apply_rules2(input: &str) -> bool {
    let bytes = input.as_bytes();

    bytes.windows(3).any(|pair| pair[0] == pair[2]) && {
        // Iterate through every pair of characters
        bytes.windows(2).enumerate().any(|(i, pair)|
            // Find the last occurence of the pattern in the string.
            input.rfind(str::from_utf8(pair).unwrap())
                // And make sure it's not sharing characters.
                .map(|index| index > i+1).unwrap_or(false))
    }
}

fn apply_rules(line: &str) -> bool {
    if ["ab", "cd", "pq", "xy"]
        .into_iter()
        .any(|x| line.find(x).is_some())
    {
        return false;
    }

    if line.matches(['a', 'e', 'i', 'o', 'u']).count() < 3 {
        return false;
    }

    line.chars().tuple_windows::<(_, _)>().any(|w| w.0 == w.1)
}

#[test]
fn test_apply() {
    assert!(apply_rules("ugknbfddgicrmopn"));
    assert!(apply_rules("aaa"));
    assert!(!apply_rules("jchzalrnumimnmhp"));
    assert!(!apply_rules("haegwjzuvuyypxyu"));
    assert!(!apply_rules("dvszwmarrgswjxmb"));
}
