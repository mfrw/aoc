use std::error::Error;
use std::fs;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input("input")?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part2(input: &[i32]) -> Result<(), Box<dyn Error>> {
    let mut prev = i32::MAX;
    let mut inc = 0;
    for i in input.windows(3) {
        let curr = i.iter().sum();
        if curr > prev {
            inc += 1;
        }
        prev = curr;
    }
    println!("Part2: {}", inc);
    Ok(())
}

fn read_input(file: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let input: String = fs::read_to_string(file)?;
    let mut vec: Vec<i32> = vec![];
    for i in input.lines() {
        vec.push(i.parse()?);
    }
    Ok(vec)
}

fn part1(input: &[i32]) -> Result<(), Box<dyn Error>> {
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
    println!("Part1: {}", inc);
    Ok(())
}
