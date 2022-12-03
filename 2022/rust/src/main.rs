#![feature(binary_heap_into_iter_sorted)]

mod utils;
use crate::utils::Solver;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Solver.solve()?;
    day2::Solver.solve()?;
    day3::Solver.solve()?;
    Ok(())
}
