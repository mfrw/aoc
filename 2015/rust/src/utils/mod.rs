use std::{fmt::Display, time::Instant};

pub fn get_input(p: &str) -> std::io::Result<String> {
    std::fs::read_to_string(p)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "File reading error"))
}

pub trait Solver<const N: usize> {
    type Part1: Display;
    type Part2: Display;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>>;
    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>>;

    fn solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(&format!("input/day{}", N))?;
        let now1 = Instant::now();
        let sol1 = self.part1(&input)?;
        let elapsed1 = now1.elapsed();
        println!("Day {} ({elapsed1:?}) Part 1: {}", N, sol1);
        let now2 = Instant::now();
        let sol2 = self.part2(&input)?;
        let elapsed2 = now2.elapsed();
        println!("Day {} ({elapsed2:?}) Part 2: {}", N, sol2);
        Ok(())
    }
}
