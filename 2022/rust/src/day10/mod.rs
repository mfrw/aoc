use std::str::FromStr;

use crate::utils;

pub struct Solver;

impl utils::Solver<10> for Solver {
    type Part1 = i32;

    type Part2 = String;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let cpu = process(input);
        Ok(cpu.sum_vals_starting(20, 40))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let cpu = process(input);

        let mut ans = String::new();
        ans.push_str("\n");
        cpu.vals
            .iter()
            .enumerate()
            .for_each(|(i, &val)| draw(&mut ans, i, val));
        Ok(ans.into())
    }
}

fn draw(ans: &mut String, i: usize, val: i32) {
    let x_pos = (i % 40) as i32;
    if val == x_pos || val == x_pos - 1 || val == x_pos + 1 {
        ans.push_str("#");
    } else {
        ans.push_str(".");
    }
    if x_pos == 39 {
        ans.push_str("\n");
    }
}

fn process(input: &str) -> CPU {
    let mut cpu = CPU { x: 1, vals: vec![] };
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .for_each(|ins| cpu.apply(&ins));
    cpu
}

enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
enum InsError {
    BadIns,
    ParseIntError,
}

impl FromStr for Instruction {
    type Err = InsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<_>>();
        match v[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                if let Ok(nr) = v[1].parse::<i32>() {
                    Ok(Instruction::Addx(nr))
                } else {
                    Err(InsError::ParseIntError)
                }
            }
            _ => Err(InsError::BadIns),
        }
    }
}

struct CPU {
    x: i32,
    vals: Vec<i32>,
}

impl CPU {
    fn apply(&mut self, i: &Instruction) {
        match i {
            Instruction::Noop => {
                self.vals.push(self.x);
            }
            Instruction::Addx(n) => {
                self.vals.push(self.x);
                self.vals.push(self.x);
                self.x += n;
            }
        }
    }

    fn value_at(&self, t: usize) -> i32 {
        self.vals[t] * t as i32
    }

    fn sum_vals_starting(&self, start: usize, step: usize) -> i32 {
        let mut tmp = 0;
        for i in (start..self.vals.len()).step_by(step) {
            tmp += self.value_at(i);
        }
        tmp
    }
}
