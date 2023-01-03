mod utils;
use utils::Solver;

mod day1;
mod day3;
mod day6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Day.print_and_solve()?;
    day3::Day.print_and_solve()?;
    day6::Day.print_and_solve()?;
    Ok(())
}
