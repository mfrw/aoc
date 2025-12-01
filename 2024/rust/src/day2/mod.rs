use crate::utils;
pub struct Solver;

impl utils::Solver<2> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}

fn is_sorted_and_within_range(ipt: &mut [usize]) -> bool {
    if ipt.is_sorted() && ipt.windows(2).all(|s| s[0] <= s[1] && s[1] - s[0] <= 3) {
        true
    } else {
        ipt.reverse();
        ipt.is_sorted() && ipt.windows(2).all(|s| s[0] <= s[1] && s[1] - s[0] <= 3)
    }
}

fn part1_int(input: &str) -> Option<usize> {
    let mut ans = 0;

    for l in input.lines() {
        let mut r = l
            .split_ascii_whitespace()
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_sorted_and_within_range(&mut r) {
            ans += 1;
        }
    }
    Some(ans)
}
