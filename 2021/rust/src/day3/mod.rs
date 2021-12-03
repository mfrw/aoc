use crate::utils;
use std::error::Error;

pub fn main() -> Result<(usize, usize), Box<dyn Error>> {
    let input = get_input()?;
    let sol1 = part1(&input);
    println!("Day3/Part1 Sol: {}", &sol1);
    let sol2 = part2(&input);
    println!("Day3/Part2 Sol: {}", &sol2);
    Ok((sol1, sol2))
}

fn part2(input: &[String]) -> usize {
    let lines = input.iter().collect::<Vec<_>>();
    let width = lines[0].len();
    let mut binary_input = lines
        .into_iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<usize>>();
    let oxy = rating(width, &mut binary_input, false);
    let co2 = rating(width, &mut binary_input, true);
    oxy * co2
}

fn rating(width: usize, numbers: &mut [usize], invert: bool) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let index = partition(&mut *numbers, |n| n & (1 << (width - 1)) != 0);
    if invert ^ (index * 2 >= numbers.len()) {
        rating(width - 1, &mut numbers[..index], invert)
    } else {
        rating(width - 1, &mut numbers[index..], invert)
    }
}

fn partition<T, P>(data: &mut [T], pred: P) -> usize
where
    P: Fn(&T) -> bool,
{
    let len = data.len();
    if len == 0 {
        return 0;
    }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && pred(&data[l]) {
            l += 1;
        }
        while r > 0 && !pred(&data[r]) {
            r -= 1;
        }
        if l >= r {
            return l;
        }
        data.swap(l, r);
    }
}

fn part1(input: &[String]) -> usize {
    let lines = input.iter().collect::<Vec<_>>();
    let width = lines[0].len();
    let binary_input = lines
        .into_iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<usize>>();
    let mcb: usize = (0..width)
        .map(|idx| 1usize << idx)
        .filter(|idx| {
            binary_input.iter().filter(|n| *n & *idx != 0).count() * 2 > binary_input.len()
        })
        .sum();
    mcb * (((1 << width) - 1) ^ mcb)
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    let input = utils::get_input("input/day3")?;
    let mut res: Vec<String> = vec![];
    for i in input.lines() {
        res.push(i.to_string());
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_test() {
        let (got1, got2) = main().unwrap();
        let want1 = 3923414;
        let want2 = 5852595;
        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }
}
