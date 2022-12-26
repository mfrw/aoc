use std::collections::HashMap;

use crate::utils;

pub struct Day;

impl utils::Solver<3> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let input = parse_input(i)?;
        Ok(part1(&input))
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let input = parse_input(i)?;
        Ok(part2(&input))
    }
}

fn advance(ch: char, mut x: i32, mut y: i32) -> (i32, i32) {
    match ch {
        '>' => x += 1,
        'v' => y -= 1,
        '^' => y += 1,
        '<' => x -= 1,
        _ => unreachable!(),
    }
    (x, y)
}

fn part1(input: &[char]) -> usize {
    let mut mp = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    mp.insert((x, y), 1);

    for ch in input {
        (x, y) = advance(*ch, x, y);
        mp.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
    }
    mp.len()
}

fn part2(input: &[char]) -> usize {
    let mut mp = HashMap::new();
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;
    mp.insert((sx, sy), 2);
    let mut santa = true;
    let mut pos: (i32, i32);

    for ch in input {
        if santa {
            santa = false;
            pos = advance(*ch, sx, sy);
            (sx, sy) = pos;
        } else {
            santa = true;
            pos = advance(*ch, rx, ry);
            (rx, ry) = pos
        }
        mp.entry(pos).and_modify(|v| *v += 1).or_insert(1);
    }

    mp.len()
}

fn parse_input(input: &str) -> std::io::Result<Vec<char>> {
    let v = input.trim().chars().collect::<Vec<_>>();
    Ok(v)
}
