use crate::utils;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let (nrs, boards) = get_input()?;
    println!("Day4/Part1 Sol: {}", part1(&nrs, &boards));
    Ok(())
}

fn part1(draws: &[i32], boards: &[Board]) -> usize {
    for i in 5..draws.len() {
        let winner = boards.iter().find_map(|b| check_board(&draws[0..i], &b));
        if let Some(score) = winner {
            return score;
        }
    }
    unreachable!()
}

fn check_board(draws: &[i32], b: &Board) -> Option<usize> {
    for i in 0..5 {
        if (0..5).all(|j| draws.contains(&b[i][j].unwrap())) {
            return Some(board_score(draws, b) * (*draws.last().unwrap() as usize));
        }
        if (0..5).all(|j| draws.contains(&b[i][j].unwrap())) {
            return Some(board_score(draws, b) * (*draws.last().unwrap() as usize));
        }
    }
    None
}

fn board_score(draws: &[i32], b: &Board) -> usize {
    b.iter()
        .flatten()
        .map(|&x| x.unwrap())
        .filter(|x| !draws.contains(x))
        .map(|x| x as usize)
        .sum()
}

// Board is an array of Option<i32> 5x5
type Board = Vec<Vec<Option<i32>>>;

fn get_input() -> Result<(Vec<i32>, Vec<Board>), Box<dyn Error>> {
    let input = utils::get_input("input/day4")?;
    let (random_nrs, boards) = input.split_once("\n\n").unwrap();

    let nrs = random_nrs
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|b| {
            b.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(str::parse::<i32>)
                        .map(Result::ok)
                        .collect()
                })
                .collect()
        })
        .collect();
    Ok((nrs, boards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input_test() {
        get_input();
    }
}
