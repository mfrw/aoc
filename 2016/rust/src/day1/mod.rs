use std::collections::HashSet;
use std::str::FromStr;

pub fn main() -> std::io::Result<()> {
    let input = get_input()?;
    println!("Day1/Part1 Sol: {}", part1(&input));
    println!("Day1/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part1(cmds: &[Cmd]) -> i32 {
    let mut position: (i32, i32) = (0, 0);

    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut facing = 0;
    for cmd in cmds {
        let trn = turn(facing, cmd);
        facing = trn.0;
        for _ in 0..trn.1 {
            position.0 += directions[facing].0;
            position.1 += directions[facing].1;
        }
    }
    position.0.abs() + position.1.abs()
}

fn part2(cmds: &[Cmd]) -> i32 {
    let mut st = HashSet::new();
    let mut position: (i32, i32) = (0, 0);

    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut facing = 0;
    for cmd in cmds {
        let trn = turn(facing, cmd);
        facing = trn.0;
        for _ in 0..trn.1 {
            position.0 += directions[facing].0;
            position.1 += directions[facing].1;
            if st.contains(&position) {
                return position.0.abs() + position.1.abs();
            } else {
                st.insert(position);
            }
        }
    }
    unreachable!()
}

fn turn(facing: usize, cmd: &Cmd) -> (usize, i32) {
    match cmd {
        Cmd::Right(v) => ((facing + 1) % 4, *v),
        Cmd::Left(v) => ((facing + 3) % 4, *v),
    }
}

fn get_input() -> std::io::Result<Vec<Cmd>> {
    let input = std::fs::read_to_string("input/day1")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> std::io::Result<Vec<Cmd>> {
    let v = input
        .trim()
        .split(", ")
        .map(|c| Cmd::from_str(c).unwrap())
        .collect::<Vec<_>>();
    Ok(v)
}

enum Cmd {
    Left(i32),
    Right(i32),
}

impl FromStr for Cmd {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("R") {
            let val = s.strip_prefix("R").unwrap().parse::<i32>()?;
            Ok(Cmd::Right(val))
        } else if s.starts_with("L") {
            let val = s.strip_prefix("L").unwrap().parse::<i32>()?;
            Ok(Cmd::Left(val))
        } else {
            unreachable!()
        }
    }
}
