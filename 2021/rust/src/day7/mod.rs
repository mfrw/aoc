use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;

    println!("Day7/Part1 Sol: {}", part1(&input));
    println!("Day7/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part1(input: &[i32]) -> usize {
    let min = *input.into_iter().min().unwrap();
    let max = *input.into_iter().max().unwrap();
    (min..=max)
        .into_iter()
        .map(|i| input.iter().map(|j| ((j - i).abs()) as usize).sum())
        .min()
        .unwrap()
}

fn part2(input: &[i32]) -> usize {
    let min = *input.into_iter().min().unwrap();
    let max = *input.into_iter().max().unwrap();
    (min..=max)
        .into_iter()
        .map(|i| {
            input
                .iter()
                .map(|j| ((j - i).abs()) as usize)
                .map(|n| n * (n + 1) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

fn get_input() -> Result<Vec<i32>, std::io::Error> {
    let input = utils::get_input("input/day7")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<i32>, std::io::Error> {
    let v = input
        .split(",")
        .map(|s| s.trim())
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let str_in = "16,1,2,0,4,2,7,1,2,14";
        let raw_input = parse_input(str_in).unwrap();
        assert_eq!(37, part1(&raw_input));
    }
    #[test]
    fn part2_test() {
        let str_in = "16,1,2,0,4,2,7,1,2,14";
        let raw_input = parse_input(str_in).unwrap();
        assert_eq!(168, part2(&raw_input));
    }
}
