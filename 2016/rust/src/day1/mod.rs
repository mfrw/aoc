use std::collections::HashSet;

use crate::utils;

pub struct Day;

impl utils::Solver<1> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut sled = Sled::new();

        for cmd in parse_input(input) {
            sled.apply(&cmd);
        }
        Ok(sled.find_manhattan((0, 0)))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut sled = Sled::new();
        let mut st = HashSet::new();

        for cmd in parse_input(input) {
            for pos in sled.apply_iterator(&cmd) {
                if st.contains(&pos) {
                    return Ok(find_manhattan((0, 0), pos));
                } else {
                    st.insert(pos);
                }
            }
        }
        unreachable!()
    }
}

fn parse_input(i: &str) -> impl Iterator<Item = Command> + '_ {
    i.split(",").map(|c| c.trim()).map(|c| {
        if c.starts_with("L") {
            let amt = c.strip_prefix("L").unwrap().parse().unwrap();
            Command::Left(amt)
        } else if c.starts_with("R") {
            let amt = c.strip_prefix("R").unwrap().parse().unwrap();
            Command::Right(amt)
        } else {
            panic!("Corrupt Input");
        }
    })
}

enum Command {
    Left(usize),
    Right(usize),
}

struct Sled {
    pos: (i32, i32),
    dir: Direction,
}

enum Direction {
    U,
    L,
    D,
    R,
}

fn find_manhattan(start: (i32, i32), end: (i32, i32)) -> usize {
    ((start.0 - end.0).abs() + (start.1 - end.1).abs()) as usize
}

impl Sled {
    fn new() -> Sled {
        Sled {
            pos: (0, 0),
            dir: Direction::U,
        }
    }

    fn find_manhattan(&self, start: (i32, i32)) -> usize {
        ((start.0 - self.pos.0).abs() + (start.1 - self.pos.1).abs()) as usize
    }

    fn set_direction_and_move(
        &mut self,
        d: Direction,
        amt: usize,
    ) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.dir = d;
        use Direction::*;
        let delta = match self.dir {
            U => (0, 1),
            R => (1, 0),
            L => (-1, 0),
            D => (0, -1),
        };
        (0..amt).into_iter().map(move |_| {
            self.pos.0 += delta.0;
            self.pos.1 += delta.1;
            (self.pos.0, self.pos.1)
        })
    }

    fn apply(&mut self, c: &Command) -> (i32, i32) {
        self.apply_iterator(c).last().unwrap()
    }

    fn apply_iterator(&mut self, c: &Command) -> impl Iterator<Item = (i32, i32)> + '_ {
        use Direction::*;
        match c {
            Command::Left(n) => match self.dir {
                U => self.set_direction_and_move(Direction::L, *n),
                L => self.set_direction_and_move(Direction::D, *n),
                D => self.set_direction_and_move(Direction::R, *n),
                R => self.set_direction_and_move(Direction::U, *n),
            },
            Command::Right(n) => match self.dir {
                U => self.set_direction_and_move(Direction::R, *n),
                L => self.set_direction_and_move(Direction::U, *n),
                D => self.set_direction_and_move(Direction::L, *n),
                R => self.set_direction_and_move(Direction::D, *n),
            },
        }
    }
}

#[test]
fn basic_movement() {
    let commands = vec![Command::Right(2), Command::Left(3)];
    let mut sled = Sled::new();
    for com in &commands {
        sled.apply(com);
    }
    assert_eq!(5, sled.find_manhattan((0, 0)));
}

#[test]
fn test_example() {
    let commands = vec![
        Command::Right(5),
        Command::Left(5),
        Command::Right(5),
        Command::Right(3),
    ];
    let mut sled = Sled::new();
    for com in &commands {
        sled.apply(com);
    }
    assert_eq!(12, sled.find_manhattan((0, 0)));
}
