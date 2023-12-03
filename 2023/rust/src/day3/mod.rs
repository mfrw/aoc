#![allow(dead_code)]
use crate::utils;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{multispace0, one_of},
    combinator::{map, map_res},
    multi::many0,
    IResult,
};
use std::collections::HashSet;

pub struct Solver;

impl utils::Solver<3> for Solver {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part2_int(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut numbers: Vec<Num> = Vec::new();
    let mut gear_map = HashSet::new();

    for (idx1, x) in grid.into_iter().enumerate() {
        let mut digit = 0;
        let mut area = HashSet::new();

        for (idx2, ch) in x.into_iter().enumerate() {
            match ch {
                '*' => {
                    gear_map.insert((idx1, idx2));
                    push_num(&mut numbers, &mut digit, &mut area);
                }
                _ if ch.is_digit(10) => {
                    add_coord(&mut area, idx1, idx2);
                    digit = digit * 10 + ch.to_digit(10).unwrap();
                }
                _ => push_num(&mut numbers, &mut digit, &mut area),
            }
        }
        push_num(&mut numbers, &mut digit, &mut area);
    }

    Some(gear_map.iter().fold(0, |mut answer, &coord| {
        let mut last_num = 0;
        let mut counter = 0;

        for n in &numbers {
            if n.area.contains(&coord) {
                if counter > 0 {
                    answer += last_num * n.num;
                    counter = 0;
                }
                last_num = n.num;
                counter += 1;
            }
        }
        answer
    }))
}

fn part1_int(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut numbers: Vec<Num> = Vec::new();
    let mut sign_map = HashSet::new();

    for (idx1, x) in grid.into_iter().enumerate() {
        let mut digit = 0;
        let mut area = HashSet::new();

        for (idx2, ch) in x.into_iter().enumerate() {
            match ch {
                '.' => push_num(&mut numbers, &mut digit, &mut area),
                _ if ch.is_digit(10) => {
                    add_coord(&mut area, idx1, idx2);
                    digit = digit * 10 + ch.to_digit(10).unwrap();
                }
                _ => {
                    push_num(&mut numbers, &mut digit, &mut area);
                    sign_map.insert((idx1, idx2));
                }
            }
        }
        push_num(&mut numbers, &mut digit, &mut area);
    }

    Some(
        numbers
            .iter()
            .filter(|n| n.area.iter().any(|coord| sign_map.contains(coord)))
            .map(|n| n.num)
            .sum(),
    )
}

fn part1_int_nom(input: &str) -> Option<usize> {
    for line in input.lines() {
        if let Ok((_, ln)) = parse_line(line) {
            println!("{:?} -> {}", ln, ln.len());
        }
    }
    Some(1)
}

#[derive(Debug)]
enum EngineItem {
    Part(usize),
    Sym(char),
}

fn parse_line(i: &str) -> IResult<&str, Vec<EngineItem>> {
    many0(alt((
        map(parse_sym, EngineItem::Sym),
        map(parse_number, EngineItem::Part),
    )))(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_sym(i: &str) -> IResult<&str, char> {
    one_of("/.@$*#&-+")(i)
}

fn sp(i: &str) -> IResult<&str, &str> {
    multispace0(i)
}

#[derive(Debug, Clone)]
struct Num {
    num: u32,
    area: HashSet<(usize, usize)>,
}

impl Num {
    fn new(n: u32, area: HashSet<(usize, usize)>) -> Self {
        Self { num: n, area }
    }
}

fn add_coord(area: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    if let Some(x1) = x.checked_sub(1) {
        area.insert((x1, y));
        area.insert((x1, y + 1));
        if let Some(y1) = y.checked_sub(1) {
            area.insert((x1, y1));
        }
    }
    if let Some(y1) = y.checked_sub(1) {
        area.insert((x, y1));
        area.insert((x + 1, y1));
    }
    area.insert((x, y + 1));
    area.insert((x + 1, y));
    area.insert((x + 1, y + 1));
}

fn push_num(numbers: &mut Vec<Num>, digit: &mut u32, area: &mut HashSet<(usize, usize)>) {
    numbers.push(Num::new(*digit, area.clone()));
    area.clear();
    *digit = 0;
}

#[test]
fn p1_test() {
    let i = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(Some(4361), part1_int(i))
}

#[test]
fn p2_test() {
    let i = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(Some(467835), part2_int(i))
}
