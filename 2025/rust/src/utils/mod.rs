use std::fmt::Display;

pub fn get_input(pth: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(pth)
}

pub trait Solver<const DAY: usize> {
    type Part1: Display;
    type Part2: Display;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>>;
    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>>;

    fn solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(&format!("input/day{}", DAY))?;
        let now1 = std::time::Instant::now();
        let sol1 = self.part1(&input)?;
        let elapsed1 = now1.elapsed();
        println!("Day{}/part1 ({:?}) Sol: {}", DAY, elapsed1, sol1);

        let now2 = std::time::Instant::now();
        let sol2 = self.part2(&input)?;
        let elapsed2 = now2.elapsed();
        println!("Day{}/part2 ({:?}) Sol: {}", DAY, elapsed2, sol2);

        Ok(())
    }
}
