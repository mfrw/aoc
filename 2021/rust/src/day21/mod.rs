use itertools::Itertools;
use std::collections::HashMap;

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day21/Part1 Sol: {}", part1(input.0, input.1));
    println!("Day21/Part2 Sol: {}", part2(input.0, input.1));
    Ok(())
}

fn part2(pos1: usize, pos2: usize) -> usize {
    play_game(&mut HashMap::new(), 0, 0, pos1, pos2, true).0
}

fn part1(pos1: usize, pos2: usize) -> usize {
    let (mut pos1, mut pos2) = (pos1, pos2);
    let (mut p1, mut p2, mut die, mut nrolls) = (0, 0, 1, 0);
    loop {
        for _ in 0..3 {
            if die > 100 {
                die = 1;
            }
            pos1 += die;
            die = (die + 1) % 100;
        }
        nrolls += 3;
        while pos1 > 10 {
            pos1 -= 10
        }
        p1 += pos1;
        if p1 >= 1000 {
            break;
        }

        for _ in 0..3 {
            if die > 100 {
                die = 1;
            }
            pos2 += die;
            die = (die + 1) % 100;
        }
        nrolls += 3;
        while pos2 > 10 {
            pos2 -= 10
        }
        p2 += pos2;
        if p2 >= 1000 {
            break;
        }
    }
    nrolls * if p1 >= 1000 { p2 } else { p1 }
}

type Cache = HashMap<(usize, usize, usize, usize, bool), (usize, usize)>;

fn play_game(
    cache: &mut Cache,
    p1: usize,
    p2: usize,
    pos1: usize,
    pos2: usize,
    turn1: bool,
) -> (usize, usize) {
    if p1 >= 21 {
        return (1, 0);
    }
    if p2 >= 21 {
        return (0, 1);
    }
    if let Some(&score) = cache.get(&(p1, p2, pos1, pos2, turn1)) {
        return score;
    }
    let mut score = (0, 0);
    for ((d1, d2), d3) in [1, 2, 3]
        .iter()
        .cartesian_product([1, 2, 3])
        .cartesian_product([1, 2, 3])
    {
        let die = d1 + d2 + d3;
        let (s1, s2) = if turn1 {
            let pos1 = pos1 + die - if pos1 + die > 10 { 10 } else { 0 };
            play_game(cache, p1 + pos1, p2, pos1, pos2, false)
        } else {
            let pos2 = pos2 + die - if pos2 + die > 10 { 10 } else { 0 };
            play_game(cache, p1, p2 + pos2, pos1, pos2, true)
        };
        score.0 += s1;
        score.1 += s2;
    }
    cache.insert((p1, p2, pos1, pos2, turn1), score);
    score
}

fn get_input() -> Result<(usize, usize), std::io::Error> {
    let input = utils::get_input("input/day21")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<(usize, usize), std::io::Error> {
    let (p1, p2) = input.trim_matches('\n').split_once('\n').unwrap();
    let pos1 = p1
        .strip_prefix("Player 1 starting position: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let pos2 = p2
        .strip_prefix("Player 2 starting position: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    Ok((pos1, pos2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(742257, part1(input.0, input.1));
    }
    #[test]
    fn part2_test() {
        let input = get_input().unwrap();
        assert_eq!(93726416205179, part2(input.0, input.1));
    }
}
