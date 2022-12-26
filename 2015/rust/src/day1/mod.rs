use crate::utils;

pub struct Day;

impl utils::Solver<1> for Day {
    type Part1 = i64;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let input = parse_input(i)?;
        Ok(part1(&input))
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let input = parse_input(i)?;
        Ok(part2(&input))
    }
}

fn part2(input: &[char]) -> usize {
    let mut ans = 0;
    for (idx, ch) in input.iter().enumerate() {
        match ch {
            '(' => ans += 1,
            ')' => ans -= 1,
            _ => unreachable!(),
        }
        if ans == -1 {
            return idx + 1;
        }
    }
    unreachable!()
}

fn part1(input: &[char]) -> i64 {
    let mut ans = 0;
    for ch in input {
        match ch {
            '(' => ans += 1,
            ')' => ans -= 1,
            _ => unreachable!(),
        }
    }
    ans
}

fn get_input() -> std::io::Result<Vec<char>> {
    let input = utils::get_input("input/day1")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> std::io::Result<Vec<char>> {
    let v = input.trim().chars().collect::<Vec<_>>();
    Ok(v)
}
