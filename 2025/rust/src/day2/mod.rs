use crate::utils;

pub struct Solver;

impl utils::Solver<2> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let ranges = input.trim().split(',').map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            start..=end
        });

        let mut count = 0;
        for range in ranges {
            for id in range {
                let id_str = id.to_string();
                if id_str.len() % 2 != 0 {
                    continue;
                }
                if &id_str[..id_str.len() / 2] == &id_str[id_str.len() / 2..] {
                    count += id;
                }
            }
        }

        Ok(count)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let ranges = input.trim().split(',').map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            start..=end
        });

        let mut count = 0;
        for range in ranges {
            for id in range {
                let id_str = id.to_string();

                if invalid_id(&id_str) {
                    count += id;
                }
            }
        }
        Ok(count)
    }
}

fn invalid_id(id: &str) -> bool {
    for i in 1..id.len() {
        if !id.len().is_multiple_of(i) {
            continue;
        }

        let substr = &id[..i];
        if id == substr.repeat(id.len() / i) {
            return true;
        }
    }

    false
}
