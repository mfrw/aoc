use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day9/Part1 Sol: {}", part1(&input));
    println!("Day9/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part2(input: &[Vec<u8>]) -> usize {
    use std::collections::HashSet;
    fn dfs(i: isize, j: isize, grid: &[Vec<u8>], visited: &mut HashSet<(isize, isize)>) {
        if i < 0
            || j < 0
            || i >= grid.len() as isize
            || j >= grid[i as usize].len() as isize
            || visited.contains(&(i, j))
            || grid[i as usize][j as usize] == 9
        {
            return;
        }
        visited.insert((i, j));
        dfs(i - 1, j, grid, visited);
        dfs(i + 1, j, grid, visited);
        dfs(i, j - 1, grid, visited);
        dfs(i, j + 1, grid, visited);
    }

    let mut visited = HashSet::new();

    let mut memo = vec![];
    for (i, row) in input.iter().enumerate() {
        for j in 0..row.len() {
            if (!visited.contains(&(i as isize, j as isize))) && input[i][j] != 9 {
                let prev = visited.len();
                dfs(i as isize, j as isize, &input, &mut visited);
                memo.push(visited.len() - prev);
            }
        }
    }
    memo.sort();
    memo.into_iter().rev().take(3).fold(1, |acc, p| acc * p)
}

fn part1(input: &[Vec<u8>]) -> usize {
    let mut copy: Vec<Vec<u8>> = vec![vec![u8::MAX; input[0].len() + 2]; input.len() + 2];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            copy[i + 1][j + 1] = input[i][j];
        }
    }
    let mut ans = vec![];

    for i in 1..copy.len() {
        for j in 1..copy[0].len() {
            if copy[i][j] < copy[i - 1][j]
                && copy[i][j] < copy[i + 1][j]
                && copy[i][j] < copy[i][j - 1]
                && copy[i][j] < copy[i][j + 1]
            {
                ans.push(copy[i][j] + 1);
            }
        }
    }
    ans.iter().map(|a| *a as usize).sum()
}

fn get_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
    let input = utils::get_input("input/day9")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_custom_test() {
        let i = "2199943210\n 3987894921\n 9856789892\n 8767896789\n 9899965678\n";
        let input = parse_input(&i).unwrap();
        assert_eq!(15, part1(&input));
    }
    #[test]
    fn part2_custom_test() {
        let i = "2199943210\n 3987894921\n 9856789892\n 8767896789\n 9899965678\n";
        let input = parse_input(&i).unwrap();
        assert_eq!(1134, part2(&input));
    }
    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(572, part1(&input));
    }
    #[test]
    fn part2_test() {
        let input = get_input().unwrap();
        assert_eq!(847044, part2(&input));
    }
}
