use crate::utils;
use std::error::Error;

mod board;
pub use board::Board;

pub fn main() -> Result<(), Box<dyn Error>> {
    let (nrs, boards) = get_input()?;
    println!("Day4/Part1 Sol: {}", part1(&nrs, boards.clone()));
    println!("Day4/Part2 Sol: {}", part2(&nrs, boards.clone()));
    Ok(())
}

fn part1(numbers: &[usize], mut boards: Vec<Board>) -> usize {
    for &nr in numbers {
        for board in boards.iter_mut() {
            board.mark_cell(nr);
            if board.has_bingo() {
                let unmarked_sum: usize = board
                    .cells
                    .iter()
                    .filter(|(_, &coord)| !board.marked_cells.contains(&coord))
                    .map(|(num, _)| *num)
                    .sum();
                return unmarked_sum * nr;
            }
        }
    }
    unreachable!()
}

fn part2(numbers: &[usize], mut boards: Vec<Board>) -> usize {
    let mut last_bingo_idx: usize = 0;
    let mut last_nr: usize = 0;

    for &nr in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            if !board.has_bingo() {
                board.mark_cell(nr);
                if board.has_bingo() {
                    last_bingo_idx = idx;
                    last_nr = nr;
                }
            }
        }
    }
    let board = &boards[last_bingo_idx];
    let unmarked_sum: usize = board
        .cells
        .iter()
        .filter(|(_, &coord)| !board.marked_cells.contains(&coord))
        .map(|(num, _)| *num)
        .sum();
    unmarked_sum * last_nr
}

fn get_input() -> Result<(Vec<usize>, Vec<Board>), Box<dyn Error>> {
    let input = utils::get_input("input/day4")?;
    let (random_nrs, boards) = input.split_once("\n\n").unwrap();

    let nrs = random_nrs
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|s| s.parse::<Board>().unwrap())
        .collect();
    Ok((nrs, boards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let (nrs, boards) = get_input().unwrap();
        let want = part1(&nrs, boards);
        assert_eq!(want, 25023);
    }
    #[test]
    fn part2_test() {
        let (nrs, boards) = get_input().unwrap();
        let want = part2(&nrs, boards);
        assert_eq!(want, 2634);
    }
}
