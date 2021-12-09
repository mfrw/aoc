use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day9/Part1 Sol: {}", part1(&input));
    Ok(())
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
    fn part1_test() {
        let i = "2199943210\n 3987894921\n 9856789892\n 8767896789\n 9899965678\n";
        let input = parse_input(&i).unwrap();
        assert_eq!(15, part1(&input));
    }
}
