use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;

use crate::utils;

pub struct Solver;

impl utils::Solver<14> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut init_state = parse_rocks(input);
        let rocks = init_state.len();
        let sand = Filled { x: 500, y: 0 };
        sand.tick_all(&mut init_state, &Part::Part1);
        Ok(init_state.len() - rocks)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut init_state = parse_rocks(input);
        let rocks = init_state.len();
        let sand = Filled { x: 500, y: 0 };
        sand.tick_all(&mut init_state, &Part::Part2);
        Ok(init_state.len() - rocks)
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Filled {
    x: usize,
    y: usize,
}
fn parse_rocks(input: &str) -> HashSet<Filled> {
    let mut filled: HashSet<Filled> = HashSet::new();

    for line in input.lines() {
        line.split(" -> ")
            .map(|l| {
                let (x, y) = if let Some((x, y)) = l.split_once(",") {
                    (x, y)
                } else {
                    panic!();
                };
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .tuple_windows::<(_, _)>()
            .for_each(|((x1, y1), (x2, y2))| {
                let start_x = min(x1, x2);
                let end_x = max(x1, x2);

                let start_y = min(y1, y2);
                let end_y = max(y1, y2);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        filled.insert(Filled { x, y });
                    }
                }
            });
    }
    return filled;
}

impl Filled {
    fn priority(&self) -> [Filled; 3] {
        [
            Filled {
                x: self.x,
                y: self.y + 1,
            },
            Filled {
                x: self.x - 1,
                y: self.y + 1,
            },
            Filled {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    fn tick(&self, fill: &mut HashSet<Filled>, limit: usize, part: &Part) -> bool {
        let mut pos = self.clone();

        loop {
            // record starting position
            let start = pos;

            // check in order of priority which open spot is empty
            for m in pos.priority() {
                if !fill.contains(&m) {
                    pos = m;
                    break;
                }
            }

            // check if we weren't able to move
            if start == pos {
                break;
            }

            match part {
                Part::Part1 => {
                    // stop for infinite falling
                    if pos.y >= limit {
                        break;
                    }
                }
                Part::Part2 => {
                    // stop for floor
                    if pos.y == limit + 1 {
                        break;
                    }
                }
            }
        }

        match part {
            Part::Part1 => {
                // fill until something fell past the lowest rock
                if pos.y >= limit {
                    return false;
                } else {
                    fill.insert(pos);
                    return true;
                }
            }
            Part::Part2 => {
                // keep inserting until we fill the starting point
                fill.insert(pos);
                return pos.y != self.y;
            }
        }
    }

    fn tick_all(&self, hash: &mut HashSet<Filled>, part: &Part) {
        let limit = hash.iter().fold(0, |acc, sand| max(acc, sand.y));

        while self.tick(hash, limit, part) {}
    }
}

enum Part {
    Part1,
    Part2,
}
