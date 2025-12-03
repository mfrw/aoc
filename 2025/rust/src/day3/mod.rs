use crate::utils;

pub struct Solver;

impl utils::Solver<3> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut ans = 0;
        for line in input.lines() {
            ans += max_batteries(line.as_bytes(), 2)
        }
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut ans = 0;
        for line in input.lines() {
            ans += max_batteries(line.as_bytes(), 12);
        }
        Ok(ans)
    }
}

fn max_batteries(xs: &[u8], l: usize) -> usize {
    let mut r = String::new();
    let mut j = 0;
    for i in 0..l {
        j = (j..xs.len() - l + i + 1)
            .max_by_key(|&x| (xs[x], usize::MAX - x))
            .unwrap();
        r.push(xs[j] as char);
        j += 1;
    }
    r.parse().unwrap()
}
