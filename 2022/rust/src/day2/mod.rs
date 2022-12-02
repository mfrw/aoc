use crate::utils;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::get_input("input/day2")?;
    let sol1 = part1(input.clone())?;
    let sol2 = part2(input)?;
    println!("Day2/Part1 Sol: {}", sol1);
    println!("Day2/Part2 Sol: {}", sol2);
    Ok(())
}

fn part1(input: String) -> Result<i32, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| match l {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        })
        .sum::<i32>())
}

fn part2(input: String) -> Result<i32, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| match l {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        })
        .sum::<i32>())
}
