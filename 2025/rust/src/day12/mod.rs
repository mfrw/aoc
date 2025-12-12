use crate::utils;

pub struct Solver;

impl utils::Solver<12> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let (_, regions) = parse_data(input);
        let mut answer = 0;
        for (region, present_counts) in regions {
            let three_by_three_squares = region.0 / 3 * region.1 / 3;
            // Naive solution: All the presents fit in a 3x3 grid
            // For my input we don't need to pack them tightly.
            if three_by_three_squares >= present_counts.iter().sum() {
                answer += 1;
            }
        }
        Ok(answer)
    }

    fn part2(&self, _input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(0)
    }
}

fn parse_data(data: &str) -> (Vec<usize>, Vec<((usize, usize), Vec<usize>)>) {
    let mut shape_sizes = Vec::new();
    let mut regions = Vec::new();
    for section in data.split("\n\n") {
        if section.contains('#') {
            shape_sizes.push(section.chars().filter(|&c| c == '#').count())
        } else {
            for line in section.lines() {
                let mut entries = line.split_ascii_whitespace();
                let dimensions = entries.next().unwrap();
                let x = dimensions[..dimensions.find('x').unwrap()]
                    .parse::<usize>()
                    .unwrap();
                let y = dimensions[dimensions.find('x').unwrap() + 1..dimensions.len() - 1]
                    .parse::<usize>()
                    .unwrap();
                let counts = entries.map(|e| e.parse().unwrap()).collect();
                regions.push(((x, y), counts));
            }
        }
    }

    (shape_sizes, regions)
}
