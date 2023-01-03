use crate::utils;

pub struct Day;

impl utils::Solver<3> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(parse_input(input).filter(|t| t.valid()).count())
    }

    fn part2(&self, _input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(0)
    }
}

fn parse_input(i: &str) -> impl Iterator<Item = Triangle> + '_ {
    i.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| Triangle::new(v[0], v[1], v[2]))
}

struct Triangle(usize, usize, usize);

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Triangle {
        let mut v = [a, b, c];
        v.sort();
        Triangle(v[0], v[1], v[2])
    }

    fn valid(&self) -> bool {
        self.0 + self.1 > self.2
    }
}
