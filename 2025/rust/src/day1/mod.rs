use crate::utils;

pub struct Solver;

impl utils::Solver<1> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut num = 50;
        let mut count = 0;
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let mut dir = -1;
            if chars[0] == 'R' {
                dir = 1;
            }
            let number = &chars[1..];
            let offset: i32 = number.iter().collect::<String>().parse()?;

            num = (num + (offset * dir)).rem_euclid(100);

            if num == 0 {
                count += 1;
            }
        }

        return Ok(count);
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}
