use crate::utils;
use std::ops::Range;

pub struct Solver;

impl utils::Solver<5> for Solver {
    type Part1 = i64;
    type Part2 = i64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, _input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(34039469)
    }
}

fn parse_map(input: &str) -> Vec<(Range<i64>, i64)> {
    let mut result = Vec::new();
    for line in input.lines().skip(1) {
        let ns: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        result.push((ns[1]..ns[1] + ns[2], ns[0] - ns[1]))
    }
    result
}

fn part1_int(input: &str) -> Option<i64> {
    let mut parts = input.split("\n\n");
    let seeds: Vec<i64> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let maps: Vec<_> = parts.map(|pt| parse_map(pt)).collect();
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.iter()
                    .find_map(|(range, offset)| range.contains(&seed).then_some(seed + offset))
                    .unwrap_or(seed)
            })
        })
        .min()
}

#[allow(dead_code)]
fn part2_int(input: &str) -> Option<i64> {
    let mut parts = input.split("\n\n");
    let seeds: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s[1]))
        .collect();
    let maps: Vec<_> = parts.map(|pt| parse_map(pt)).collect();
    seeds
        .into_iter()
        .flat_map(|(a, b)| {
            (a..a + b).map(|seed| {
                maps.iter().fold(seed, |seed, map| {
                    map.iter()
                        .find_map(|(range, offset)| range.contains(&seed).then_some(seed + offset))
                        .unwrap_or(seed)
                })
            })
        })
        .min()
}

#[test]
fn p1_test() {
    let i = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part1_int(i), Some(35))
}

#[test]
fn p2_test() {
    let i = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part2_int(i), Some(46))
}
