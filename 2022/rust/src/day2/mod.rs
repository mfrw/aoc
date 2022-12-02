use crate::utils;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn evaluate(opponent: &Shape, player: &Shape) -> Self {
        match (opponent, player) {
            (Shape::Rock, Shape::Rock) => Self::Draw,
            (Shape::Rock, Shape::Paper) => Self::Win,
            (Shape::Rock, Shape::Scissors) => Self::Loss,
            (Shape::Paper, Shape::Rock) => Self::Loss,
            (Shape::Paper, Shape::Paper) => Self::Draw,
            (Shape::Paper, Shape::Scissors) => Self::Win,
            (Shape::Scissors, Shape::Rock) => Self::Win,
            (Shape::Scissors, Shape::Paper) => Self::Loss,
            (Shape::Scissors, Shape::Scissors) => Self::Draw,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn from_outcome(op: &Shape, outcome: &Outcome) -> Self {
        match (op, outcome) {
            (Shape::Rock, Outcome::Win) => Self::Paper,
            (Shape::Rock, Outcome::Draw) => Self::Rock,
            (Shape::Rock, Outcome::Loss) => Self::Scissors,
            (Shape::Paper, Outcome::Win) => Self::Scissors,
            (Shape::Paper, Outcome::Draw) => Self::Paper,
            (Shape::Paper, Outcome::Loss) => Self::Rock,
            (Shape::Scissors, Outcome::Win) => Self::Rock,
            (Shape::Scissors, Outcome::Draw) => Self::Scissors,
            (Shape::Scissors, Outcome::Loss) => Self::Paper,
        }
    }
}

fn part1_internal(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|l| {
            let v = l
                .split_whitespace()
                .map(|s| s.parse::<Shape>().unwrap())
                .collect::<Vec<_>>();
            Some((v[0], v[1]))
        })
        .map(|(o, p)| Outcome::evaluate(&o, &p).score() + p.value())
        .sum()
}

fn part2_internal(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|l| Some(l.split_whitespace().collect::<Vec<_>>()))
        .map(|v| {
            (
                v[0].parse::<Shape>().unwrap(),
                v[1].parse::<Outcome>().unwrap(),
            )
        })
        .map(|(op, oc)| Shape::from_outcome(&op, &oc).value() + oc.score())
        .sum()
}

pub struct Solver;

impl utils::Solver<2> for Solver {
    type Part1 = i32;

    type Part2 = i32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_internal(input))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_internal(input))
    }
}
