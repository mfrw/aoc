use crate::utils;

pub struct Solver;

impl utils::Solver<3> for Solver {
    type Part1 = u32;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut ans = 0;
        for line in input.lines() {
            let mut mx = 0;
            let c: Vec<u32> = line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            for i in 0..c.len() {
                for j in (i + 1)..c.len() {
                    mx = mx.max(c[i] * 10 + c[j]);
                }
            }
            ans += mx;
        }
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}
