use crate::utils;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let sol1 = part1(&input)?;
    let sol2 = part2(&input)?;
    println!("Day1/Part1 Sol: {}", sol1);
    println!("Day1/Part2 Sol: {}", sol2);
    Ok(())
}

fn part2(input: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut prev = i32::MAX;
    let mut inc = 0;
    for i in input.windows(3) {
        let curr = i.iter().sum();
        if curr > prev {
            inc += 1;
        }
        prev = curr;
    }
    Ok(inc)
}

fn get_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let input = utils::get_input("input/day1")?;

    let mut vec: Vec<i32> = vec![];
    for i in input.lines() {
        vec.push(i.parse()?);
    }
    Ok(vec)
}

fn part1(input: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut iter = input.iter();
    let mut prev = iter.next().unwrap();

    let mut inc = 0;

    for i in iter {
        let cur = i;
        if cur > prev {
            inc += 1;
        }
        prev = cur;
    }
    Ok(inc)
}
