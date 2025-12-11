use crate::utils;
use std::collections::HashMap;

pub struct Solver;

impl utils::Solver<11> for Solver {
    type Part1 = u64;

    type Part2 = u64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let grid = parse_input(input).unwrap();
        let mut memo = HashMap::new();
        let result = count_paths("you", "out", &grid, &mut memo);
        Ok(result)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let grid = parse_input(input).unwrap();
        let mut memo = HashMap::new();

        // find paths from srv -> dac -> fft -> out
        let p1 = count_paths("svr", "dac", &grid, &mut memo)
            * count_paths("dac", "fft", &grid, &mut memo)
            * count_paths("fft", "out", &grid, &mut memo);
        // find paths from srv -> fft -> dac -> out
        let p2 = count_paths("svr", "fft", &grid, &mut memo)
            * count_paths("fft", "dac", &grid, &mut memo)
            * count_paths("dac", "out", &grid, &mut memo);

        Ok(p1 + p2)
    }
}

fn parse_input(input: &str) -> Option<HashMap<&str, Vec<&str>>> {
    let mut mp = HashMap::new();
    for l in input.lines() {
        if let Some((k, v)) = l.split_once(':') {
            let mut vals = v.trim().split_whitespace().collect::<Vec<_>>();
            mp.entry(k)
                .and_modify(|val: &mut Vec<&str>| val.append(&mut vals))
                .or_insert(vals);
        }
    }
    Some(mp)
}

fn count_paths<'a>(
    curr: &'a str,
    target: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if curr == target {
        return 1;
    }
    let key = (curr, target);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(curr) {
        for &nxt in neighbors {
            total += count_paths(nxt, target, graph, memo);
        }
    }

    memo.insert(key, total);
    total
}
