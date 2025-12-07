use crate::utils;
use std::collections::HashSet;

pub struct Solver;

type Coord = usize;
type Pos = (Coord, Coord);

impl utils::Solver<7> for Solver {
    type Part1 = u32;

    type Part2 = u32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let (start, height, splitters) = parse_input(input);
        Ok(count_splits(start, height, &splitters))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}

fn count_splits(start: Pos, height: Coord, splitters: &HashSet<Pos>) -> u32 {
    let mut count = 0;
    let (x, y0) = start;
    let mut beams: HashSet<Coord> = HashSet::from([x; 1]);
    for y in y0 + 1..height {
        let mut new_beams: HashSet<Coord> = HashSet::new();
        for x in beams {
            if splitters.contains(&(x, y)) {
                count += 1;
                new_beams.insert(x - 1);
                new_beams.insert(x + 1);
            } else {
                new_beams.insert(x);
            }
        }
        beams = new_beams;
    }
    count
}

fn parse_input(input: &str) -> (Pos, Coord, HashSet<Pos>) {
    let mut start: Pos = (0, 0);
    let mut splitters: HashSet<Pos> = HashSet::new();
    let mut height = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (j, i);
            } else if ch == '^' {
                splitters.insert((j, i));
            }
        }
        height = i + 1;
    }
    (start, height, splitters)
}
