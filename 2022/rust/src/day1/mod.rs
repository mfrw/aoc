use crate::utils;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::get_input("input/day1")?;
    let sol1 = part1(input.clone())?;
    println!("Day1/Part1 Sol: {}", sol1);
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
