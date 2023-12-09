use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};

use crate::utils;
use std::collections::HashMap;

pub struct Solver;

impl<'a> utils::Solver<8> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

struct Map<'a> {
    instructions: Vec<Dir>,
    edges: HashMap<&'a str, Edges<'a>>,
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Map<'a>> {
        let (input, instructions) = many1(one_of("LR"))(input)?;
        let instructions = instructions.iter().map(|c| Dir::new(*c)).collect();
        let (input, _) = many1(newline)(input)?;

        let mut edges = HashMap::new();
        let (input, nodes) = separated_list0(
            tag("\n"),
            tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")"))),
        )(input)?;

        nodes.iter().for_each(|(node, _, left, _, right, _)| {
            edges.insert(*node, Edges { left, right });
        });
        Ok((
            input,
            Map {
                instructions,
                edges,
            },
        ))
    }

    fn find_end<F>(&'a self, start: &'a str, end: F) -> (usize, &'a str)
    where
        F: Fn(&str) -> bool,
    {
        let mut steps = 0;
        let mut cur = start;
        for dir in self.instructions.iter().cycle() {
            let edges = self.edges.get(cur).unwrap();
            cur = match dir {
                Dir::L => edges.left,
                Dir::R => edges.right,
            };
            steps += 1;
            if end(cur) {
                break;
            }
        }
        (steps, cur)
    }
}

enum Dir {
    L,
    R,
}

impl Dir {
    fn new(c: char) -> Self {
        match c {
            'L' | 'l' => Dir::L,
            'R' | 'r' => Dir::R,
            _ => unreachable!(),
        }
    }
}

struct Edges<'a> {
    left: &'a str,
    right: &'a str,
}

fn part1_int(input: &str) -> Option<usize> {
    let (start, end) = ("AAA", "ZZZ");

    if let Ok((_, map)) = Map::parse(input) {
        Some(map.find_end(start, |s| s == end).0)
    } else {
        None
    }
}

fn part2_int(input: &str) -> Option<usize> {
    todo!()
}

#[test]
fn p1_test() {
    let i = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(Some(6), part1_int(i))
}
