use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day8/Part1 Sol: {}", part1(&input));
    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("|").unwrap();
            let x: Vec<&str> = a.trim().split_whitespace().collect();
            let y: Vec<&str> = b.trim().split_whitespace().collect();
            (x, y)
        })
        .map(|(_, output)| {
            output
                .iter()
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn get_input() -> Result<String, std::io::Error> {
    utils::get_input("input/day8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(294, part1(&input));
    }
}
