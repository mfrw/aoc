mod utils;
use utils::Solver;

mod day1;
mod day3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Day.print_and_solve()?;
    day3::Day.print_and_solve()?;
    Ok(())
}
