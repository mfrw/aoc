use std::cmp::Ordering;
use std::collections::HashSet;

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let (pts, ins) = get_input()?;
    println!("Day13/Part1 Sol: {}", part1(&pts, &ins));
    Ok(())
}

fn fold_n(pts: &[(i32, i32)], ins: &[(char, i32)], nr: usize) -> usize {
    let mut st = pts.into_iter().map(|x| *x).collect::<HashSet<_>>();
    for (ori, inst) in ins.into_iter().take(nr) {
        st = st
            .into_iter()
            .flat_map(|(x, y)| match (ori, x.cmp(&inst), y.cmp(&inst)) {
                ('x', Ordering::Less, _) | ('y', _, Ordering::Less) => Some((x, y)),
                ('x', Ordering::Greater, _) => Some((2 * inst - x, y)),
                ('y', _, Ordering::Greater) => Some((x, 2 * inst - y)),
                _ => None,
            })
            .collect();
    }
    st.len()
}

fn part1(pts: &[(i32, i32)], ins: &[(char, i32)]) -> usize {
    fold_n(pts, ins, 1)
}

fn get_input() -> Result<(Vec<(i32, i32)>, Vec<(char, i32)>), std::io::Error> {
    let input = utils::get_input("input/day13")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<(Vec<(i32, i32)>, Vec<(char, i32)>), std::io::Error> {
    let (raw_pt, raw_ins) = input.split_once("\n\n").unwrap();

    let pts = raw_pt
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let ins = raw_ins
        .lines()
        .map(|l| {
            let raw_inst = l
                .strip_prefix("fold along ")
                .unwrap()
                .split_once("=")
                .unwrap();
            (
                raw_inst.0.chars().last().unwrap(),
                raw_inst.1.parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    Ok((pts, ins))
}
