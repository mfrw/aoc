use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

use crate::utils;

pub struct Solver;

impl utils::Solver<18> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let st: HashSet<Point> = HashSet::from_iter(parse_input(input));
        let sfa = surface_area(&st);
        Ok(sfa)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let st: HashSet<Point> = HashSet::from_iter(parse_input(input));

        let min_x = st.iter().map(|p| p.x).min().unwrap();
        let min_y = st.iter().map(|p| p.y).min().unwrap();
        let min_z = st.iter().map(|p| p.z).min().unwrap();
        let max_x = st.iter().map(|p| p.x).max().unwrap();
        let max_y = st.iter().map(|p| p.y).max().unwrap();
        let max_z = st.iter().map(|p| p.z).max().unwrap();

        let x_range = min_x - 1..max_x + 2;
        let y_range = min_y - 1..max_y + 2;
        let z_range = min_z - 1..max_z + 2;
        let mut water: HashSet<Point> = HashSet::new();
        let mut q: VecDeque<Point> =
            VecDeque::from([(x_range.start, y_range.start, z_range.start).into()]);

        while let Some(cube) = q.pop_front() {
            if x_range.contains(&cube.x) && y_range.contains(&cube.y) && z_range.contains(&cube.z) {
                for adj in cube.adjacent() {
                    if !st.contains(&adj) && water.insert(adj.clone()) {
                        q.push_back(adj);
                    }
                }
            }
        }
        Ok(st
            .par_iter()
            .map(|cube| {
                cube.adjacent()
                    .par_bridge()
                    .filter(|x| water.contains(x))
                    .count()
            })
            .sum())
    }
}

fn surface_area(st: &HashSet<Point>) -> usize {
    st.par_iter()
        .map(|cube| {
            cube.adjacent()
                .par_bridge()
                .filter(|adj| !st.contains(&adj))
                .count()
        })
        .sum()
}

fn parse_input(i: &str) -> impl Iterator<Item = Point> + '_ {
    i.lines().map(|l| {
        let (x, y, z) = scan_fmt!(l, "{},{},{}", i64, i64, i64).unwrap();
        Point { x, y, z }
    })
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<(i64, i64, i64)> for Point {
    fn from(value: (i64, i64, i64)) -> Self {
        Point {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn adjacent(&self) -> impl Iterator<Item = Point> + '_ {
        let Point { x, y, z } = self;
        (0..6)
            .into_iter()
            .map(|idx| match idx {
                0 => Some(Point::new(*x - 1, *y, *z)),
                1 => Some(Point::new(*x + 1, *y, *z)),
                2 => Some(Point::new(*x, *y - 1, *z)),
                3 => Some(Point::new(*x, *y + 1, *z)),
                4 => Some(Point::new(*x, *y, *z - 1)),
                5 => Some(Point::new(*x, *y, *z + 1)),
                _ => None,
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
    }
}
