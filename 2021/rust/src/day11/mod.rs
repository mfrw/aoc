use std::collections::VecDeque;

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day11/Part1 Sol: {}", part1(&mut (input.clone())));
    println!("Day11/Part2 Sol: {}", part2(&mut (input.clone())));
    Ok(())
}

fn part2(input: &mut [Vec<u8>]) -> usize {
    if input.is_empty() {
        return 0;
    }
    let max_row = input.len();
    let max_col = input[0].len();
    for step in 1.. {
        let mut flashes = 0;
        let mut q = VecDeque::new();
        let mut mark = vec![vec![false; 10]; 10];
        for i in 0..max_row {
            for j in 0..max_col {
                input[i][j] += 1;
                if input[i][j] > 9 {
                    input[i][j] = 0;
                    mark[i][j] = true;
                    q.push_back((i as i32, j as i32));
                    flashes += 1;
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            for (x, y) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x, y) = (i + x, j + y);
                if x >= 0
                    && x < max_row as i32
                    && y >= 0
                    && y < max_col as i32
                    && !mark[x as usize][y as usize]
                {
                    input[x as usize][y as usize] += 1;
                    if input[x as usize][y as usize] > 9 {
                        input[x as usize][y as usize] = 0;
                        mark[x as usize][y as usize] = true;
                        q.push_back((x, y));
                        flashes += 1;
                    }
                }
            }
        }
        if flashes == max_row * max_col {
            return step;
        }
    }
    unreachable!()
}

fn part1(input: &mut [Vec<u8>]) -> usize {
    if input.is_empty() {
        return 0;
    }
    let max_row = input.len();
    let max_col = input[0].len();
    let mut flashes = 0;
    for _ in 0..100 {
        let mut q = VecDeque::new();
        let mut mark = vec![vec![false; 10]; 10];
        for i in 0..max_row {
            for j in 0..max_col {
                input[i][j] += 1;
                if input[i][j] > 9 {
                    input[i][j] = 0;
                    mark[i][j] = true;
                    q.push_back((i as i32, j as i32));
                    flashes += 1;
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            for (x, y) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x, y) = (i + x, j + y);
                if x >= 0
                    && x < max_row as i32
                    && y >= 0
                    && y < max_col as i32
                    && !mark[x as usize][y as usize]
                {
                    input[x as usize][y as usize] += 1;
                    if input[x as usize][y as usize] > 9 {
                        input[x as usize][y as usize] = 0;
                        mark[x as usize][y as usize] = true;
                        q.push_back((x, y));
                        flashes += 1;
                    }
                }
            }
        }
    }
    flashes
}

fn get_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
    let input = utils::get_input("input/day11")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_custom_test() {
        let input = "11111 \n19991 \n19191 \n19991 \n11111";
        let got = part1(&mut parse_input(input).unwrap());
        assert_eq!(259, got);
    }
    #[test]
    fn part1_test() {
        let mut input = get_input().unwrap();
        let got = part1(&mut input);
        assert_eq!(1603, got);
    }
    #[test]
    fn part2_custom_test() {
        let input = "11111 \n19991 \n19191 \n19991 \n11111";
        let got = part2(&mut parse_input(input).unwrap());
        assert_eq!(6, got);
    }
    #[test]
    fn part2_test() {
        let mut input = get_input().unwrap();
        let got = part2(&mut input);
        assert_eq!(222, got);
    }
}
