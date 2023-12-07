use crate::utils;
use std::ops::RangeInclusive;

pub struct Solver;

impl utils::Solver<6> for Solver {
    type Part1 = u64;
    type Part2 = u64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part1_int(input: &str) -> Option<u64> {
    let input = parse_p1(input);
    let ans = input
        .iter()
        .map(|r| r.possible_win_range().count())
        .reduce(|a, b| a * b)
        .unwrap() as u64;
    Some(ans)
}

fn part2_int(input: &str) -> Option<u64> {
    todo!()
}

struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn possible_win_range(&self) -> RangeInclusive<u64> {
        let mut min = None;
        let mut max = None;
        for i in 1..=self.time {
            let velocity = i;
            let time_left = self.time - i;

            let distance = velocity * time_left;

            if distance > self.record_distance {
                if min.is_none() {
                    min = Some(i);
                }
                max = Some(i);
            }
        }

        min.unwrap()..=max.unwrap()
    }
}

fn parse_p1(input: &str) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0];
    let distances = lines[1];

    let times = times.strip_prefix("Time:").unwrap().trim();
    let distances = distances.strip_prefix("Distance:").unwrap().trim();

    let times = times.split_whitespace().map(|s| s.parse::<u64>().unwrap());
    let distances = distances
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    times
        .zip(distances)
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect()
}

#[test]
fn p1_test() {
    let i = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(Some(288), part1_int(i))
}
