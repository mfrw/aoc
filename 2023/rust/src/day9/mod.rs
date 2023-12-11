use crate::utils;

pub struct Solver;

impl utils::Solver<9> for Solver {
    type Part1 = i64;

    type Part2 = i64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part1_int(input: &str) -> Option<i64> {
    Some(solve(
        &parse_input(input),
        |n| n[n.len() - 1],
        extrapolate_last,
    ))
}

fn part2_int(input: &str) -> Option<i64> {
    Some(solve(&parse_input(input), |n| n[0], extrapolate_first))
}

pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve(input: &Vec<Vec<i64>>, select: fn(&[i64]) -> i64, extrapolate: fn(&[i64]) -> i64) -> i64 {
    let mut sum = 0;

    let mut nums = vec![];
    let mut next = vec![];
    let mut vals = vec![];

    for line in input.iter() {
        vals.clear();
        nums.clear();
        nums.extend_from_slice(&line);

        loop {
            next.clear();
            vals.push(select(&nums));

            let mut more_to_process = false;
            for w in nums.windows(2) {
                let diff = w[1] - w[0];
                more_to_process |= diff != 0;
                next.push(diff);
            }

            if !more_to_process {
                break;
            }

            std::mem::swap(&mut nums, &mut next);
        }

        sum += extrapolate(&vals);
    }

    sum
}

fn extrapolate_last(nums: &[i64]) -> i64 {
    let mut ext = 0;

    for idx in (0..nums.len()).rev() {
        ext = nums[idx] + ext;
    }

    ext
}

fn extrapolate_first(nums: &[i64]) -> i64 {
    let mut ext = 0;

    for idx in (0..nums.len()).rev() {
        ext = nums[idx] - ext
    }

    ext
}

#[test]
fn p1_test() {
    let i = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part1_int(i), Some(114))
}

#[test]
fn p2_test() {
    let i = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part2_int(i), Some(2))
}
