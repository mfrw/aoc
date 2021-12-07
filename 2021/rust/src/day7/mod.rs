use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;

    println!("Day7/Part1 Sol: {}", part1(&input));
    Ok(())
}

fn part1(input: &[i32]) -> usize {
    let min = *input.into_iter().min().unwrap();
    let max = *input.into_iter().max().unwrap();

    let mut costs = vec![0usize];
    for i in 1..(max - min + 1) {
        costs.push(i as usize);
    }
    let mut best = usize::MAX;
    for i in min..max {
        let mut tmp = 0;
        for j in input {
            let cost = (i - j).abs() as usize;
            tmp += cost;
        }
        best = std::cmp::min(tmp, best);
    }
    best
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
}
