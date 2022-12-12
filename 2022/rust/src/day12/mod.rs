use std::collections::{HashSet, VecDeque};

use crate::utils::grid::GridCoord;
use crate::utils::{self, grid::Grid};

pub struct Solver;

impl utils::Solver<12> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let (grid, start, end) = parse_grid(input);
        let mut q = VecDeque::from(vec![(start.unwrap(), 0, 0)]);
        let mut st = HashSet::from([start.unwrap()]);

        while !q.is_empty() {
            let (curr, curr_ele, pth_len) = q.pop_front().unwrap();
            if curr == end.unwrap() {
                return Ok(pth_len);
            }
            for (neigh_pos, neigh_ele) in grid.neighbors(curr, false) {
                if (neigh_ele <= 1 + curr_ele) && !st.contains(&neigh_pos) {
                    q.push_back((neigh_pos, neigh_ele, pth_len + 1));
                    st.insert(neigh_pos);
                }
            }
        }
        unreachable!()
    }

    // start from the goal and reach the least elevation possible
    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let (grid, _, end) = parse_grid(input);
        let mut q = VecDeque::from(vec![(end.unwrap(), (b'z' - b'a') as usize, 0)]);
        let mut st = HashSet::from([end.unwrap()]);

        while !q.is_empty() {
            let (curr, curr_ele, pth_len) = q.pop_front().unwrap();
            if curr_ele == 0 {
                return Ok(pth_len);
            }
            for (neigh_pos, neigh_ele) in grid.neighbors(curr, false) {
                if (curr_ele <= 1 + neigh_ele) && !st.contains(&neigh_pos) {
                    q.push_back((neigh_pos, neigh_ele, pth_len + 1));
                    st.insert(neigh_pos);
                }
            }
        }
        unreachable!()
    }
}

fn parse_grid(input: &str) -> (Grid<usize>, Option<GridCoord>, Option<GridCoord>) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            let val = match col {
                'S' => {
                    start = Some((x, y).into());
                    0
                }
                'E' => {
                    end = Some((x, y).into());
                    (b'z' - b'a') as usize
                }
                'a'..='z' => (col as u8 - b'a') as usize,
                _ => panic!("Bad input"),
            };
            *grid.cell_mut((x, y).into()).unwrap() = val;
        }
    }
    (grid, start, end)
}
