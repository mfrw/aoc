use std::collections::HashSet;

use crate::utils;

pub struct Solver;

impl utils::Solver<10> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn fill_map(
    input: &str,
) -> (
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    (usize, usize),
) {
    let dat = input.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let fr = dat
        .iter()
        .enumerate()
        .find_map(|(y, &a)| a.iter().position(|&c| c == b'S').map(|x| (x, y)))
        .unwrap();
    let (wid, hgt) = (dat[0].len(), dat.len());
    let mut p =
        fr.0.checked_sub(1)
            .filter(|&x| "-FL".contains(dat[fr.1][x] as char))
            .map(|x| (x, fr.1))
            .or_else(|| {
                fr.1.checked_sub(1)
                    .filter(|&y| "|7F".contains(dat[y][fr.0] as char))
                    .map(|y| (fr.0, y))
            })
            .unwrap_or_else(|| (fr.0, fr.1 + 1));
    let mut pr = fr;
    let mut pt = HashSet::new();
    pt.insert(p);
    let mut nx = HashSet::new();
    loop {
        if let Some(q) = (pr.0 + pr.1)
            .checked_sub(p.1)
            .zip((pr.1 + p.0).checked_sub(pr.0))
            .filter(|&(x, y)| x < wid && y < hgt)
        {
            nx.insert(q);
        }
        if let Some(q) = (p.0 + pr.1)
            .checked_sub(p.1)
            .zip((p.1 + p.0).checked_sub(pr.0))
            .filter(|&(x, y)| x < wid && y < hgt)
        {
            nx.insert(q);
        }
        if p == fr {
            break;
        }
        let q = match dat[p.1][p.0] {
            b'|' => {
                if p.1 < pr.1 {
                    (p.0, p.1 - 1)
                } else {
                    (p.0, p.1 + 1)
                }
            }
            b'7' => {
                if p.1 < pr.1 {
                    (p.0 - 1, p.1)
                } else {
                    (p.0, p.1 + 1)
                }
            }
            b'F' => {
                if p.1 < pr.1 {
                    (p.0 + 1, p.1)
                } else {
                    (p.0, p.1 + 1)
                }
            }
            b'L' => {
                if p.1 > pr.1 {
                    (p.0 + 1, p.1)
                } else {
                    (p.0, p.1 - 1)
                }
            }
            b'J' => {
                if p.1 > pr.1 {
                    (p.0 - 1, p.1)
                } else {
                    (p.0, p.1 - 1)
                }
            }
            b'-' => {
                if p.0 < pr.0 {
                    (p.0 - 1, p.1)
                } else {
                    (p.0 + 1, p.1)
                }
            }
            _ => panic!(),
        };
        (p, pr) = (q, p);
        pt.insert(p);
    }
    (pt, nx, (wid, hgt))
}

fn part1_int(input: &str) -> Option<usize> {
    let pt = fill_map(input).0;
    Some(pt.len() / 2)
}

fn part2_int(input: &str) -> Option<usize> {
    let (pt, mut nx, (wid, hgt)) = fill_map(input);
    let mut sd = HashSet::new();
    loop {
        nx.retain(|p| !sd.contains(p) && !pt.contains(p));
        if nx.is_empty() {
            break;
        }
        let mut a = HashSet::new();
        for p in nx {
            sd.insert(p);
            if p.0 > 0 {
                a.insert((p.0 - 1, p.1));
            }
            if p.1 > 0 {
                a.insert((p.0, p.1 - 1));
            }
            if p.0 < wid - 1 {
                a.insert((p.0 + 1, p.1));
            }
            if p.1 < hgt - 1 {
                a.insert((p.0, p.1 + 1));
            }
        }
        nx = a;
    }
    let f = sd
        .iter()
        .all(|&(x, y)| 0 < x && x < wid - 1 && 0 < y && y < hgt - 1);
    let ans = if f {
        sd.len()
    } else {
        wid * hgt - sd.len() - pt.len()
    };
    Some(ans)
}

#[test]
fn p1_test() {
    let i = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(part1_int(i), Some(4))
}

#[test]
fn p2_test() {
    let i = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    assert_eq!(part2_int(i), Some(10))
}
