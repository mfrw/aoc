use crate::utils::{self, Result};

pub fn main() -> Result<()> {
    let input = get_input()?;
    println!("Day3/Part1 : {}", part1(&input));
    println!("Day3/Part2 : {}", part2(&input));
    Ok(())
}

fn tree_counter(input: &[Vec<char>], rt: usize, dn: usize) -> usize {
    let row = input.len();
    let col = input[0].len();
    let (mut i, mut j) = (dn, rt);
    let mut count = 0;
    while i < row {
        if input[i][j] == '#' {
            count += 1;
        }
        i += dn;
        j = (j + rt) % col;
    }
    count
}

fn part2(input: &[Vec<char>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(rt, dn)| tree_counter(input, rt, dn))
        .fold(1, |acc, cnt| acc * cnt)
}

fn part1(input: &[Vec<char>]) -> usize {
    tree_counter(input, 3, 1)
}

fn get_input() -> Result<Vec<Vec<char>>> {
    let input = utils::get_input("input/day3")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    let v = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    Ok(v)
}
