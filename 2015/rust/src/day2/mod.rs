use crate::utils;

pub struct Day;

impl utils::Solver<2> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let input = parse_input(i)?;
        Ok(part1(&input))
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let input = parse_input(i)?;
        Ok(part2(&input))
    }
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
