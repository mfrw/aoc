use crate::utils;

pub struct Solver;

impl utils::Solver<4> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let rolls: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let ans = round(&rolls, 4);
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}

const D: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn round(rolls: &[Vec<char>], arg: usize) -> usize {
    let mut res = 0;
    for r in 0..rolls.len() {
        for c in 0..rolls[0].len() {
            if rolls[r][c] != '@' {
                continue;
            }
            let n = D
                .iter()
                .filter(|&&(dr, dc)| {
                    rolls
                        .get((r as i64 + dr) as usize)
                        .and_then(|row| row.get((c as i64 + dc) as usize))
                        .is_some_and(|&x| x == '@')
                })
                .count();
            if n < arg {
                res += 1;
            }
        }
    }
    res
}
