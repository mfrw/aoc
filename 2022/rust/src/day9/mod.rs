use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::utils;

pub struct Solver;

impl utils::Solver<9> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut state = State::default();
        state.mp.insert(state.tail_ptr, 1);
        input
            .lines()
            .map(|l| l.parse::<Command>().unwrap())
            .for_each(|c| state.process(&c));

        Ok(state.mp.len())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut state = Pos { x: 0, y: 0 };
        let mut tails = [Pos { x: 0, y: 0 }; 9];
        let mut visited = HashSet::new();
        input
            .lines()
            .map(|l| l.parse::<Command>().unwrap())
            .for_each(|c| {
                for _ in 0..c.nr {
                    match c.d {
                        Direction::Up => state.y += 1,
                        Direction::Down => state.y -= 1,
                        Direction::Right => state.x += 1,
                        Direction::Left => state.x -= 1,
                    }
                    follow(&state, &mut tails[0]);
                    for i in 1..9 {
                        let (left, right) = tails.split_at_mut(i);
                        follow(&left[i - 1], &mut right[0]);
                    }
                    visited.insert(tails[8]);
                }
            });
        Ok(visited.len())
    }
}

#[derive(Debug, Default)]
struct State {
    head_ptr: (i32, i32),
    tail_ptr: (i32, i32),
    mp: HashMap<(i32, i32), usize>,
}

impl State {
    fn process(&mut self, cmd: &Command) {
        for _ in 0..cmd.nr {
            match cmd.d {
                Direction::Up => self.head_ptr.1 += 1,
                Direction::Down => self.head_ptr.1 -= 1,
                Direction::Right => self.head_ptr.0 += 1,
                Direction::Left => self.head_ptr.0 -= 1,
            }
            self.pull_tail();
            self.mp
                .entry(self.tail_ptr)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    fn pull_tail(&mut self) {
        if self
            .head_ptr
            .0
            .abs_diff(self.tail_ptr.0)
            .max(self.head_ptr.1.abs_diff(self.tail_ptr.1))
            > 1
        {
            match self.head_ptr.0.cmp(&self.tail_ptr.0) {
                Ordering::Greater => self.tail_ptr.0 += 1,
                Ordering::Less => self.tail_ptr.0 -= 1,
                Ordering::Equal => {}
            }

            match self.head_ptr.1.cmp(&self.tail_ptr.1) {
                Ordering::Greater => self.tail_ptr.1 += 1,
                Ordering::Less => self.tail_ptr.1 -= 1,
                Ordering::Equal => {}
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Command {
    d: Direction,
    nr: usize,
}

#[derive(Debug)]
enum CommandError {
    BadCommand,
    ParseIntError(std::num::ParseIntError),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CommandError {}

impl From<std::num::ParseIntError> for CommandError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<_>>();
        let nr = v[1].parse::<usize>()?;
        match v[0] {
            "U" => Ok(Command {
                d: Direction::Up,
                nr,
            }),
            "D" => Ok(Command {
                d: Direction::Down,
                nr,
            }),
            "L" => Ok(Command {
                d: Direction::Left,
                nr,
            }),
            "R" => Ok(Command {
                d: Direction::Right,
                nr,
            }),
            _ => Err(CommandError::BadCommand),
        }
    }
}

#[derive(Copy, Eq, PartialEq, Clone, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn follow(lead: &Pos, t: &mut Pos) {
    let dx = lead.x - t.x;
    let dy = lead.y - t.y;
    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        t.x += dx.signum();
        t.y += dy.signum()
    }
}
