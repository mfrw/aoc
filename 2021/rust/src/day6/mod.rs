use crate::utils;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    println!("Day6/Part1 Sol: {}", part1(&input));
    println!("Day6/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part1(input: &[i32]) -> usize {
    lantern_fish_generations(input, 80)
}

fn part2(input: &[i32]) -> usize {
    lantern_fish_generations(input, 256)
}

fn lantern_fish_generations(input: &[i32], days: usize) -> usize {
    let mut fish_count = [0usize; 9];
    for d in input {
        fish_count[*d as usize] += 1;
    }
    (0..days).for_each(|_| {
        fish_count.rotate_left(1);
        fish_count[6] += fish_count[8];
    });
    fish_count.iter().sum()
}

fn get_input() -> Result<Vec<i32>, std::io::Error> {
    let input = utils::get_input("input/day6")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<i32>, std::io::Error> {
    let v = input
        .split(",")
        .map(|s| s.trim())
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(388739, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = get_input().unwrap();
        assert_eq!(1741362314973, part2(&input));
    }
}
