use std::collections::HashMap;

use crate::utils;

pub struct Day;

impl utils::Solver<6> for Day {
    type Part1 = String;

    type Part2 = String;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let ans = solve1(input);
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let ans = solve2(input);
        Ok(ans)
    }
}

fn solve1(input: &str) -> String {
    let grid = parse_input(input);

    let mut ans = vec![];
    for r in 0..grid[0].len() {
        let mut mp = HashMap::new();
        for c in 0..grid.len() {
            let ch = grid[c][r];
            mp.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        let (k, _) = mp.into_iter().max_by_key(|(_, v)| *v).unwrap();
        ans.push(k);
    }
    ans.into_iter().collect()
}

fn solve2(input: &str) -> String {
    let grid = parse_input(input);

    let mut ans = vec![];
    for r in 0..grid[0].len() {
        let mut mp = HashMap::new();
        for c in 0..grid.len() {
            let ch = grid[c][r];
            mp.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        let (k, _) = mp.into_iter().min_by_key(|(_, v)| *v).unwrap();
        ans.push(k);
    }
    ans.into_iter().collect()
}

fn parse_input(i: &str) -> Vec<Vec<char>> {
    i.lines().map(|l| l.chars().collect::<Vec<_>>()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

    #[test]
    fn part1_example() {
        assert_eq!("easter", solve1(INPUT))
    }

    #[test]
    fn part2_example() {
        assert_eq!("advent", solve2(INPUT))
    }
}
