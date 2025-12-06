use crate::utils;

pub struct Solver;

impl utils::Solver<6> for Solver {
    type Part1 = i64;

    type Part2 = i64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let (nums, ops) = parse_input(input).unwrap();
        let mut cols = vec![];
        for (idx, op) in ops.into_iter().enumerate() {
            match op {
                '*' => {
                    let mut col = 1;
                    for i in 0..nums.len() {
                        col *= nums[i][idx];
                    }
                    cols.push(col);
                }
                '+' => {
                    let mut col = 0;
                    for i in 0..nums.len() {
                        col += nums[i][idx];
                    }
                    cols.push(col);
                }
                _ => unreachable!(),
            }
        }
        Ok(cols.into_iter().sum())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let characters: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut stack: Vec<i64> = Vec::new();
        let mut result = 0;

        'next_column: for x in (0..characters[0].len()).rev() {
            let mut is_number = false;
            let mut current_number = 0;

            for row in &characters {
                match row[x] {
                    x if x.is_ascii_digit() => {
                        is_number = true;
                        current_number = 10 * current_number + x.to_digit(10).unwrap() as i64;
                    }
                    '+' => {
                        stack.push(current_number);

                        result += stack.iter().sum::<i64>();
                        stack.clear();
                        continue 'next_column;
                    }
                    '*' => {
                        stack.push(current_number);

                        result += stack.iter().product::<i64>();
                        stack.clear();
                        continue 'next_column;
                    }
                    _ => {}
                }
            }

            if is_number {
                stack.push(current_number);
            }
        }
        Ok(result)
    }
}

fn parse_input(input: &str) -> Option<(Vec<Vec<i64>>, Vec<char>)> {
    let mut nums = vec![];
    let mut ops = vec![];
    for l in input.lines() {
        let l = l.trim();
        if l.starts_with('*') || l.starts_with('+') {
            ops = l
                .split_whitespace()
                .map(|c| c.parse::<char>().unwrap())
                .collect::<Vec<_>>();
        } else {
            let n = l
                .split_whitespace()
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            nums.push(n);
        }
    }
    Some((nums, ops))
}
