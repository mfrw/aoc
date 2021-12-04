use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    pub cells: HashMap<usize, Coordinate>,
    pub marked_cells: HashSet<Coordinate>,
    pub bingo: bool,
}

impl FromStr for Board {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        s.lines().enumerate().for_each(|(row, line)| {
            line.split_whitespace()
                .enumerate()
                .for_each(|(col, marker)| {
                    let num = marker.parse::<usize>().unwrap();
                    board.cells.insert(num, (row, col));
                })
        });
        Ok(board)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            marked_cells: HashSet::new(),
            bingo: false,
        }
    }

    pub fn mark_cell(&mut self, num: usize) {
        if let Some(cell) = self.cells.get(&num) {
            self.marked_cells.insert(*cell);
            self.bingo = self.has_bingo();
        }
    }

    pub fn has_bingo(&self) -> bool {
        let marked_cells = &self.marked_cells;
        if marked_cells.len() < 5 {
            return false;
        }
        for row in 0..5 {
            if (0..5).all(|col| marked_cells.contains(&(row, col))) {
                return true;
            }
        }
        for col in 0..5 {
            if (0..5).all(|row| marked_cells.contains(&(row, col))) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_from_string() {
        let s = " 1  2  3  4  5\n 6  7  8  9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";

        let cells: HashMap<usize, Coordinate> = (0..25).map(|n| (n + 1, (n / 5, n % 5))).collect();

        let board = Board {
            cells,
            marked_cells: HashSet::new(),
            bingo: false,
        };

        assert_eq!(Board::from_str(s).unwrap(), board);
    }

    #[test]
    fn test_board_has_bingo() {
        let s = " 1  2  3  4  5\n 6  7  8  9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";
        let mut board = Board::from_str(s).unwrap();

        // mark all nums in top row
        for i in 1..6 {
            board.mark_cell(i);
        }

        assert!(board.bingo);
    }
}
