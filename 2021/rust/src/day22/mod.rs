use crate::utils;
use itertools::Itertools;
use std::cmp::{max, min};

type Cube = (bool, (i64, i64), (i64, i64), (i64, i64));

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day22/Part1 Sol: {}", part1(&input));
    Ok(())
}

fn part1(cubes: &[Cube]) -> i64 {
    let sub = cubes
        .iter()
        .filter_map(|&c| subcube(c, (true, (-50, 50), (-50, 50), (-50, 50))))
        .collect::<Vec<_>>();
    total_volume(&sub)
}

fn corrected_volume(c: Cube, rest: &[Cube]) -> i64 {
    let subcubes = rest
        .iter()
        .filter_map(|&c2| subcube(c2, c))
        .collect::<Vec<_>>();
    let vsubcubes = (0..subcubes.len())
        .map(|i| corrected_volume(subcubes[i], &subcubes[i + 1..]))
        .sum::<i64>();
    volume(c) - vsubcubes
}

fn volume((_, (x0, x1), (y0, y1), (z0, z1)): Cube) -> i64 {
    (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1)
}

fn subaxis((a, b): (i64, i64), (low, high): (i64, i64)) -> Option<(i64, i64)> {
    if b < low {
        return None;
    }
    if a > high {
        return None;
    }
    let a = min(max(a, low), high);
    let b = min(max(b, low), high);
    Some((a, b))
}

fn subcube(c1: Cube, c2: Cube) -> Option<Cube> {
    let xr = subaxis(c1.1, c2.1)?;
    let yr = subaxis(c1.2, c2.2)?;
    let zr = subaxis(c1.3, c2.3)?;
    Some((c1.0, xr, yr, zr))
}

fn total_volume(cubes: &[Cube]) -> i64 {
    (0..cubes.len())
        .filter(|&i| cubes[i].0)
        .map(|i| corrected_volume(cubes[i], &cubes[i + 1..]))
        .sum()
}

fn get_input() -> Result<Vec<Cube>, std::io::Error> {
    let input = utils::get_input("input/day22")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Cube>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| {
            let (on, rest) = l.split_once(' ').unwrap();
            let (x, y, z) = rest
                .split(',')
                .map(|s| {
                    s[2..]
                        .split("..")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            (on == "on", x, y, z)
        })
        .collect::<Vec<_>>();
    Ok(v)
}
