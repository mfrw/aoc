use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    let graph = make_graph(&input);
    println!("Day12/Part1 Sol: {}", part1(&graph));
    println!("Day12/Part2 Sol: {}", part2(&graph));
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

    if src.chars().all(char::is_lowercase) {
        seen.insert(src.to_string());
    }

    let ans = graph[src]
        .iter()
        .map(|from| count_paths(graph, from, dst, seen))
        .sum();

    seen.remove(src);
    ans
}

fn count_paths_double(
    graph: &HashMap<String, Vec<String>>,
    src: &str,
    mut seen: HashSet<String>,
    mut double: bool,
) -> usize {
    if seen.contains(src) {
        if double {
            return 0;
        } else {
            double = true;
        }
    }

    if src == "end" {
        return 1;
    }

    if src.chars().all(char::is_lowercase) {
        seen.insert(src.to_string());
    }

    graph[src]
        .iter()
        .filter(|&node| node != "start")
        .map(|from| count_paths_double(graph, from, seen.clone(), double))
        .sum()
}

fn part1(graph: &HashMap<String, Vec<String>>) -> usize {
    count_paths(graph, "start", "end", &mut HashSet::new())
}
fn part2(graph: &HashMap<String, Vec<String>>) -> usize {
    count_paths_double(graph, "start", HashSet::new(), false)
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
    #[test]
    fn part2_test() {
        let graph = get_graph();
        assert_eq!(117509, part2(&graph));
    }
}
