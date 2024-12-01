mod utils;

use crate::utils::Solver;

mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Solver.solve()?;
    Ok(())
}
