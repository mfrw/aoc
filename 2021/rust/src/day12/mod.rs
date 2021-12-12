use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    let graph = make_graph(&input);
    println!("Day12/Part1 Sol: {}", part1(&graph));
    Ok(())
}

fn make_graph(input: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut mp = HashMap::new();
    input.iter().for_each(|edge| {
        mp.entry(edge.0.to_string())
            .and_modify(|e: &mut Vec<String>| e.push(edge.1.to_string()))
            .or_insert([edge.1.to_string()].into());

        mp.entry(edge.1.to_string())
            .and_modify(|e: &mut Vec<String>| e.push(edge.0.to_string()))
            .or_insert([edge.0.to_string()].into());
    });
    mp
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    src: &str,
    dst: &str,
    seen: &mut HashSet<String>,
) -> usize {
    if seen.contains(src) {
        return 0;
    }

    if src == dst {
        return 1;
    }

    if src.chars().all(|ch| ch.is_lowercase()) {
        seen.insert(src.to_string());
    }

    let ans = graph[src]
        .iter()
        .map(|from| count_paths(graph, from, dst, seen))
        .sum();

    seen.remove(src);
    ans
}

fn part1(graph: &HashMap<String, Vec<String>>) -> usize {
    count_paths(graph, "start", "end", &mut HashSet::new())
}

fn get_input() -> Result<Vec<(String, String)>, std::io::Error> {
    let input = utils::get_input("input/day12")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once("-").unwrap();
            (from.to_string(), to.to_string())
        })
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_graph() -> HashMap<String, Vec<String>> {
        let input = get_input().unwrap();
        make_graph(&input)
    }

    #[test]
    fn part1_test() {
        let graph = get_graph();
        assert_eq!(4573, part1(&graph));
    }
}
