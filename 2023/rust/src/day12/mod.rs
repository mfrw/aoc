use std::collections::HashMap;

use crate::utils;

pub struct Solver;

impl utils::Solver<12> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part1_int(input: &str) -> Option<usize> {
    Some(arrangements(input))
}

fn part2_int(input: &str) -> Option<usize> {
    todo!()
}

fn arrangements(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (conditions, damaged_str) = l.split_once(' ').unwrap();
            let damaged = damaged_str
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<usize>>();
            let a = line_arrangements(
                0,
                0,
                conditions.len() - damaged.iter().skip(1).map(|d| d + 1).sum::<usize>(),
                &conditions.chars().collect::<Vec<char>>(),
                &damaged,
                &mut HashMap::default(),
            );
            a
        })
        .sum()
}

fn line_arrangements(
    damaged_idx: usize,
    start: usize,
    end: usize,
    conditions: &[char],
    damaged: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(a) = cache.get(&(damaged_idx, start)) {
        return *a;
    }
    let size = damaged[damaged_idx];
    let last = end - size + 1;
    let mut result = 0;
    for idx in start..last {
        if idx > 0 && conditions[idx - 1] == '#' {
            break;
        } else if idx + size < conditions.len() && conditions[idx + size] == '#'
            || conditions[idx..idx + size].iter().any(|c| *c == '.')
        {
            continue;
        }

        if damaged_idx == damaged.len() - 1 {
            if conditions[idx + size..].iter().any(|c| *c == '#') {
                continue;
            }
            result += 1;
        } else {
            let next = damaged[damaged_idx + 1];
            result += line_arrangements(
                damaged_idx + 1,
                idx + size + 1,
                end + next + 1,
                conditions,
                damaged,
                cache,
            )
        }
    }
    cache.insert((damaged_idx, start), result);
    result
}
