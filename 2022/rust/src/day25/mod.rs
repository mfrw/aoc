use crate::utils;

pub struct Solver;

impl utils::Solver<25> for Solver {
    type Part1 = String;

    type Part2 = String;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let ans: i64 = parse_input(input);
        Ok(tosnafu(ans))
    }

    fn part2(&self, _input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok("No Required".into())
    }
}

fn parse_input(i: &str) -> i64 {
    i.lines().map(|l| todec(l)).sum()
}

fn todec(s: &str) -> i64 {
    s.chars().fold(0, |n, d| {
        n * 5
            + match d {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            }
    })
}

fn tosnafu(n: i64) -> String {
    if n == 0 {
        "".to_string()
    } else {
        tosnafu((n + 2) / 5) + ["0", "1", "2", "=", "-"][n as usize % 5]
    }
}
