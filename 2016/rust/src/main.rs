mod utils;
use utils::Solver;

mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Day.print_and_solve()?;
    Ok(())
}
