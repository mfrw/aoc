use crate::utils;

pub struct Solver;

impl utils::Solver<4> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut rolls: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let ans = round(&mut rolls, 4, false);
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut rolls: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let mut ans = 0;
        loop {
            let n = round(&mut rolls, 4, true);
            ans += n;
            if n == 0 {
                break;
            }
        }
        Ok(ans)
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

fn round(rolls: &mut [Vec<char>], arg: usize, remove: bool) -> usize {
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
                if remove {
                    rolls[r][c] = '.';
                }
                res += 1;
            }
        }
    }
    res
}
