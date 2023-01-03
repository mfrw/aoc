use std::fmt::Display;

pub trait Solver<const DAY: usize> {
    type Part1: Display;
    type Part2: Display;
    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>>;
    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>>;

    fn solve(&self) -> Result<(Self::Part1, Self::Part2), Box<dyn std::error::Error>> {
        let path = format!("input/day{DAY}");
        let input = std::fs::read_to_string(&path)?;
        Ok((self.part1(&input)?, self.part2(&input)?))
    }

    fn print_and_solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("input/day{DAY}");
        let input = std::fs::read_to_string(&path)?;
        let now1 = std::time::Instant::now();
        let p1 = self.part1(&input)?;
        println!("Day{} ({:?}) Part1: {}", DAY, now1.elapsed(), p1);
        let now2 = std::time::Instant::now();
        let p2 = self.part2(&input)?;
        println!("Day{} ({:?}) Part2: {}", DAY, now2.elapsed(), p2);
        Ok(())
    }
}
