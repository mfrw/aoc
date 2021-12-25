use crate::utils;

pub fn main() -> std::io::Result<()> {
    let input = get_input()?;
    println!("Day2/Part1 Sol: {}", part1(&input));
    println!("Day2/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part2(input: &[Vec<usize>]) -> usize {
    input.iter().map(|v| calc_ribbon(v[0], v[1], v[2])).sum()
}

fn part1(input: &[Vec<usize>]) -> usize {
    input.iter().map(|v| calc_area(v[0], v[1], v[2])).sum()
}

fn calc_ribbon(l: usize, w: usize, h: usize) -> usize {
    let mn = [l + w, w + h, h + l].into_iter().min().unwrap();
    l * w * h + (mn * 2)
}

fn calc_area(l: usize, w: usize, h: usize) -> usize {
    let area = 2 * (l * w + w * h + h * l);
    let mn = [l * w, w * h, h * l].into_iter().min().unwrap();
    area + mn
}

fn get_input() -> std::io::Result<Vec<Vec<usize>>> {
    let input = utils::get_input("input/day2")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> std::io::Result<Vec<Vec<usize>>> {
    let v = input
        .lines()
        .map(|l| {
            l.split("x")
                .map(|dim| usize::from_str_radix(dim, 10))
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(v)
}
