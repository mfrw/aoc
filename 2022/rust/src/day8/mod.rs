use crate::utils;

pub struct Solver;

impl utils::Solver<8> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let grid = parse_input(input);
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visible = 2 * (rows + cols) - 4;
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                let height = grid[row][col];
                if (0..col).all(|i| grid[row][i] < height)
                    || (col + 1..cols).all(|i| grid[row][i] < height)
                    || (0..row).all(|r| grid[r][col] < height)
                    || (row + 1..rows).all(|r| grid[r][col] < height)
                {
                    visible += 1;
                }
            }
        }
        Ok(visible)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let grid = parse_input(input);
        let mut score = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                let height = grid[row][col];
                let left = (0..col)
                    .enumerate()
                    .rev()
                    .find(|(_, c)| grid[row][*c] >= height)
                    .map(|(index, _)| col - index)
                    .unwrap_or(col);
                let up = (0..row)
                    .enumerate()
                    .rev()
                    .find(|(_, r)| grid[*r][col] >= height)
                    .map(|(index, _)| row - index)
                    .unwrap_or(row);
                let right = (col + 1..cols)
                    .enumerate()
                    .find(|(_, c)| grid[row][*c] >= height)
                    .map(|(index, _)| index + 1)
                    .unwrap_or(cols - col - 1);
                let down = (row + 1..rows)
                    .enumerate()
                    .find(|(_i, r)| grid[*r][col] >= height)
                    .map(|(index, _)| index + 1)
                    .unwrap_or(rows - row - 1);
                let s = left * up * right * down;
                score = score.max(s);
            }
        }
        Ok(score)
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect()
}
