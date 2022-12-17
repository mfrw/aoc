use std::collections::{HashMap, HashSet};

use crate::utils;

pub struct Solver;

impl utils::Solver<16> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(solve_a(input))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(solve_b(input))
    }
}

const MAX_ROOMS: usize = 60;

type DistanceM = [[usize; MAX_ROOMS]; MAX_ROOMS];

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Room {
    flow_rate: usize,
    neighbors: Vec<usize>,
}

#[derive(Debug)]
struct Cave {
    aa_idx: usize,
    rooms: Vec<Room>,
    name2idx: HashMap<String, usize>,
}

impl Cave {
    fn new() -> Cave {
        let rooms = Vec::with_capacity(MAX_ROOMS);
        let name2idx = HashMap::new();
        Cave {
            aa_idx: usize::MAX,
            rooms,
            name2idx,
        }
    }

    fn calc_distances(&self) -> DistanceM {
        let mut dist = [[usize::MAX; MAX_ROOMS]; MAX_ROOMS];
        let mut seen = HashSet::new();

        for (i, _) in self
            .rooms
            .iter()
            .enumerate()
            .filter(|(i, r)| r.flow_rate > 0 || *i == self.aa_idx)
        {
            let mut current = HashSet::new();
            current.insert(i);
            let mut next = HashSet::new();
            let mut d = 0;

            dist[i][i] = 0;
            while !current.is_empty() {
                d += 1;
                for pos in &current {
                    for newpos in &self.rooms[*pos].neighbors {
                        if !seen.contains(&(i, *newpos)) {
                            next.insert(*newpos);
                            dist[i][*newpos] = d;
                            seen.insert((i, *newpos));
                        }
                    }
                }
                current.clear();
                current.extend(next.drain());
            }
        }

        dist
    }
}

fn parse_input(input: &str) -> Cave {
    let (_, name2idx, mut idx2room) = input.lines().fold(
        (0usize, HashMap::new(), HashMap::new()),
        |(mut idx, mut name2idx, mut idx2room), line| {
            let (a, b) = line.split_once(';').unwrap();
            let valve = &a[6..8];
            let flow_rate = a[a.find('=').unwrap() + 1..].parse::<usize>().unwrap();

            let vid = *name2idx.entry(valve.to_string()).or_insert_with(|| {
                idx += 1;
                idx
            });

            let mut neighbors = Vec::new();
            let parts = b
                .split_ascii_whitespace()
                .skip(4)
                .map(|x| x.trim().replace(',', ""))
                .collect::<Vec<_>>();

            for x in parts {
                let z = *name2idx.entry(x).or_insert_with(|| {
                    idx += 1;
                    idx
                });
                neighbors.push(z);
            }

            idx2room.insert(
                vid,
                Room {
                    flow_rate,
                    neighbors,
                },
            );

            (idx, name2idx, idx2room)
        },
    );

    let mut cave = Cave::new();
    let nrooms = idx2room.len();
    cave.aa_idx = *name2idx.get("AA").unwrap();
    cave.name2idx = name2idx;

    // Push empty room since room idx begins at 1
    cave.rooms.push(Room {
        flow_rate: usize::MAX,
        neighbors: vec![],
    });

    for idx in 1..=nrooms {
        let r = idx2room.remove(&idx).unwrap();
        cave.rooms.push(r);
    }

    cave
}

fn find_max_release(
    dist: &DistanceM,
    cave: &Cave,
    current: usize,
    time: usize,
    targets: &mut HashSet<usize>,
) -> (usize, HashSet<usize>) {
    targets.remove(&current);

    let mut max_flow = 0;
    let mut best_path = HashSet::new();

    for t in targets.iter() {
        let time_remain = time.saturating_sub(dist[current][*t]).saturating_sub(1);
        if time_remain > 0 {
            let mut flow = cave.rooms[*t].flow_rate * time_remain;
            let (newflow, p) = find_max_release(dist, cave, *t, time_remain, &mut targets.clone());
            flow += newflow;

            if flow > max_flow {
                max_flow = flow;
                let mut tmp = p.clone();
                tmp.insert(current);
                best_path = tmp;
            }
        }
    }
    (max_flow, best_path)
}

pub fn solve_a(input: &str) -> usize {
    let cave = parse_input(input);
    let distances = cave.calc_distances();

    let mut targets = HashSet::from_iter(
        cave.rooms
            .iter()
            .enumerate()
            .filter(|(i, r)| r.flow_rate > 0 || *i == cave.aa_idx)
            .map(|(i, _)| i),
    );

    let (x, _) = find_max_release(&distances, &cave, cave.aa_idx, 30, &mut targets);

    x
}

pub fn solve_b(input: &str) -> usize {
    // This fails on the example data as the human takes all of the work
    // In all honesty I'm not sure that this should work given I assumed that
    // the human takes the optimal path and the elephant just optimizes on the
    // remaining network after removing the path the human took. blahh

    let cave = parse_input(input);
    let distances = cave.calc_distances();
    //
    // let mut a = cave.name2idx.iter().collect::<Vec<_>>();
    // a.sort_unstable_by(|a, b| b.1.cmp(a.1));
    // for (k, v) in a {
    //     println!("{}: {}", v, k);
    // }

    let mut targets = HashSet::from_iter(
        cave.rooms
            .iter()
            .enumerate()
            .filter(|(i, r)| r.flow_rate > 0 || *i == cave.aa_idx)
            .map(|(i, _)| i),
    );

    let (human_release, human_path) =
        find_max_release(&distances, &cave, cave.aa_idx, 26, &mut targets);

    let mut cave_elephant = Cave::new();
    cave_elephant.aa_idx = cave.aa_idx;
    cave_elephant.name2idx = cave.name2idx.clone();

    for (ri, r) in cave.rooms.iter().enumerate() {
        let re = Room {
            flow_rate: r.flow_rate,
            neighbors: r
                .neighbors
                .iter()
                .filter(|n| !human_path.contains(n))
                .copied()
                .collect(),
        };

        if human_path.contains(&ri) && ri != cave.aa_idx {
            cave_elephant.rooms.push(Room {
                flow_rate: 0,
                neighbors: vec![],
            });
        } else {
            cave_elephant.rooms.push(re);
        }
    }

    let mut targets_elephant = HashSet::from_iter(
        cave_elephant
            .rooms
            .iter()
            .enumerate()
            .filter(|(i, r)| r.flow_rate > 0 || *i == cave_elephant.aa_idx)
            .map(|(i, _)| i),
    );

    let (elephant_release, _) = find_max_release(
        &distances,
        &cave_elephant,
        cave_elephant.aa_idx,
        26,
        &mut targets_elephant,
    );

    let x = human_release + elephant_release;

    x
}
