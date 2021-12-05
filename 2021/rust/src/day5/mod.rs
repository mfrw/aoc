use crate::utils;
use std::collections::HashMap;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    println!("Day5/Part1 Sol: {}", part1(&input));
    Ok(())
}

type Coord = (i32, i32);

fn part1(input: &[(Coord, Coord)]) -> usize {
    let mut seen = HashMap::<Coord, i32>::new();
    for (from, to) in input {
        let x1 = i32::min(from.0, to.0);
        let x2 = i32::max(from.0, to.0);
        let y1 = i32::min(from.1, to.1);
        let y2 = i32::max(from.1, to.1);

        if y1 == y2 {
            for x in x1..=x2 {
                seen.entry((x, y1)).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        if x1 == x2 {
            for y in y1..=y2 {
                seen.entry((x1, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    seen.values().filter(|v| **v > 1).count()
}

fn get_input() -> Result<Vec<(Coord, Coord)>, std::io::Error> {
    let input = utils::get_input("input/day5")?
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split("->").map(|s| s.trim()).collect();
            let from = coord_from_str(parts[0]).unwrap();
            let to = coord_from_str(parts[1]).unwrap();
            (from, to)
        })
        .collect::<Vec<_>>();

    Ok(input)
}

fn coord_from_str(s: &str) -> Result<Coord, std::io::Error> {
    let a = s
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    Ok((a[0], a[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let v = input
            .lines()
            .map(|l| {
                let parts = l.split("->").map(|s| s.trim()).collect::<Vec<_>>();
                let from = coord_from_str(parts[0]).unwrap();
                let to = coord_from_str(parts[0]).unwrap();
                (from, to)
            })
            .collect::<Vec<_>>();
        let got = part1(&v);
        assert_eq!(got, 9);
    }
}
