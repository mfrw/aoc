#![allow(dead_code)]
use itertools::Itertools;
use std::collections::HashSet;

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    let st = make_set_and_dist(input);
    println!("Day19/Part1 Sol: {}", part1(&st.0));
    println!("Day19/Part2 Sol: {}", part2(&st.1));

    Ok(())
}

fn make_set_and_dist(mut scans: Vec<Vec<[i32; 3]>>) -> (HashSet<[i32; 3]>, Vec<[i32; 3]>) {
    let mut total_scan = scans.remove(0).into_iter().collect::<HashSet<_>>();
    let mut dists = Vec::new();
    while !scans.is_empty() {
        for i in (0..scans.len()).rev() {
            if let Some(d) = merge_scan(&mut total_scan, &scans[i]) {
                dists.push(d);
                scans.swap_remove(i);
            }
        }
    }
    (total_scan, dists)
}

fn part1(st: &HashSet<[i32; 3]>) -> usize {
    st.len()
}

fn part2(st: &[[i32; 3]]) -> i32 {
    st.iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap()
}

fn get_input() -> Result<Vec<Vec<[i32; 3]>>, std::io::Error> {
    let input = utils::get_input("input/day19")?;
    parse_input(&input)
}

fn convert_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn parse_input(input: &str) -> Result<Vec<Vec<[i32; 3]>>, std::io::Error> {
    let scans = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| l.split(',').map(|sp| sp.parse().unwrap()).collect())
                .map(convert_to_array)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(scans)
}

fn rotate([x, y, z]: [i32; 3], rot: u8) -> [i32; 3] {
    match rot {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => unreachable!(),
    }
}

fn merge_scan(total_scan: &mut HashSet<[i32; 3]>, scan: &[[i32; 3]]) -> Option<[i32; 3]> {
    for rot in 0..24 {
        let rotated_scan = scan.iter().map(|&v| rotate(v, rot)).collect::<Vec<_>>();
        let distances = total_scan
            .iter()
            .cartesian_product(&rotated_scan)
            .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);
        for [dx, dy, dz] in distances {
            let translated = rotated_scan
                .iter()
                .map(|[x, y, z]| [x + dx, y + dy, z + dz]);
            if translated
                .clone()
                .filter(|v| total_scan.contains(v))
                .count()
                >= 12
            {
                total_scan.extend(translated);
                return Some([dx, dy, dz]);
            }
        }
    }
    None
}
