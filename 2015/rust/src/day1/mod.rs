use crate::utils;

pub fn main() -> std::io::Result<()> {
    let input = get_input()?;
    println!("Day1/Part1 Sol: {}", part1(&input));
    println!("Day1/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part2(input: &[char]) -> usize {
    let mut ans = 0;
    for (idx, ch) in input.iter().enumerate() {
        match ch {
            '(' => ans += 1,
            ')' => ans -= 1,
            _ => unreachable!(),
        }
        if ans == -1 {
            return idx + 1;
        }
    }
    unreachable!()
}

fn part1(input: &[char]) -> i64 {
    let mut ans = 0;
    for ch in input {
        match ch {
            '(' => ans += 1,
            ')' => ans -= 1,
            _ => unreachable!(),
        }
    }
    ans
}

fn get_input() -> std::io::Result<Vec<char>> {
    let input = utils::get_input("input/day1")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> std::io::Result<Vec<char>> {
    let v = input.trim().chars().collect::<Vec<_>>();
    Ok(v)
}
