use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::utils;

pub struct Solver;

impl utils::Solver<7> for Solver {
    type Part1 = i64;

    type Part2 = i64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let fs = parse_fn(input);
        let mut sizes = HashMap::new();
        for k in fs.keys() {
            compute_dir_size(&fs, &mut sizes, k);
        }
        let p1 = sizes.values().filter(|&&s| s <= 100000).sum::<i64>();
        Ok(p1)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let fs = parse_fn(input);
        let mut sizes = HashMap::new();
        for k in fs.keys() {
            compute_dir_size(&fs, &mut sizes, k);
        }
        let total_size = sizes[&PathBuf::from("/")];
        let p2 = sizes
            .values()
            .filter(|&&s| 40000000 + s >= total_size)
            .min()
            .copied()
            .unwrap();
        Ok(p2)
    }
}

fn parse_fn(input: &str) -> HashMap<PathBuf, HashSet<(i64, &str)>> {
    let mut fs = HashMap::new();
    let mut pwd = PathBuf::new();
    for l in input.split('$').skip(1) {
        match l.trim().lines().next().unwrap() {
            "ls" => {
                let entries = l.lines().skip(1).map(|output| {
                    let (x, f) = output.split_once(' ').unwrap();
                    let size = if x == "dir" {
                        -1
                    } else {
                        x.parse::<i64>().unwrap()
                    };
                    (size, f)
                });
                fs.entry(pwd.clone())
                    .or_insert(HashSet::new())
                    .extend(entries);
            }
            "cd .." => {
                pwd.pop();
            }
            cd_dir => {
                pwd.push(cd_dir.split_once(' ').unwrap().1);
            }
        }
    }
    fs
}

fn compute_dir_size<'a>(
    fs: &HashMap<PathBuf, HashSet<(i64, &'a str)>>,
    sizes: &mut HashMap<PathBuf, i64>,
    dir: &PathBuf,
) {
    if sizes.contains_key(dir) {
        return;
    }
    let size = fs[dir]
        .iter()
        .map(|&(s, d)| match s {
            -1 => {
                let dir = dir.join(d);
                compute_dir_size(fs, sizes, &dir);
                sizes[&dir]
            }
            s => s,
        })
        .sum();
    sizes.insert(dir.clone(), size);
}
