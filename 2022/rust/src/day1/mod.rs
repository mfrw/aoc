use crate::utils;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::get_input("input/day1")?;
    let sol1 = part1(input.clone())?;
    let sol2 = part2(input)?;
    println!("Day1/Part1 Sol: {}", sol1);
    println!("Day1/Part2 Sol: {}", sol2);
    Ok(())
}

fn part1(input: String) -> Result<i32, Box<dyn Error>> {
    let mut mx = i32::MIN;
    for elf in input.split("\n\n") {
        let tmp: i32 = elf.lines().map(|l| l.parse::<i32>().unwrap()).sum();
        if tmp > mx {
            mx = tmp;
        }
    }
    Ok(mx)
}

fn part2(input: String) -> Result<i32, Box<dyn Error>> {
    let mut v = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    v.sort();
    Ok(v.into_iter().rev().take(3).sum())
}
