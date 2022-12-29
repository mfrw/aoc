use utils::Solver;

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Day.solve()?;
    day2::Day.solve()?;
    day3::Day.solve()?;
    day4::Day.solve()?;
    day5::Day.solve()?;
    day6::Day.solve()?;
    Ok(())
}
