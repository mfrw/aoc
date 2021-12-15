use crate::utils;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    crd: (usize, usize),
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.crd.cmp(&other.crd))
    }
}

fn shortest_path(matrix: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> usize {
    let mut dist = vec![vec![usize::MAX; matrix[0].len()]; matrix.len()]; // max cost memo
    let mut q = BinaryHeap::new();
    q.push(State {
        crd: start,
        cost: 0,
    });
    while let Some(State { crd: (x, y), cost }) = q.pop() {
        if (x, y) == end {
            return cost;
        }
        if cost > dist[x][y] {
            continue;
        }

        for (x1, y1) in [
            (x.saturating_sub(1), y),
            (x.saturating_add(1), y),
            (x, y.saturating_sub(1)),
            (x, y.saturating_add(1)),
        ] {
            if matrix.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }
            let next = State {
                cost: cost + matrix[x1][y1] as usize,
                crd: (x1, y1),
            };
            if next.cost < dist[x1][y1] {
                q.push(next);
                dist[x1][y1] = next.cost;
            }
        }
    }
    0
}

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day15/Part1 Sol: {}", part1(&input));
    Ok(())
}

fn part1(matrix: &[Vec<u8>]) -> usize {
    if matrix.is_empty() {
        return 0;
    }
    let max_i = matrix.len();
    let max_j = matrix[0].len();
    shortest_path(matrix, (0, 0), (max_i - 1, max_j - 1))
}

fn get_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
    let input = utils::get_input("input/day15")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| {
            // split line in to Vec<u8>
            l.chars()
                .map(|ch| ch.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(388, part1(&input));
    }
}
