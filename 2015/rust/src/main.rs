use utils::Solver;

mod utils;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Day.solve()?;
    day2::Day.solve()?;
    day3::Day.solve()?;
    Ok(())
}
