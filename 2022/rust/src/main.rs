#![feature(binary_heap_into_iter_sorted)]

mod utils;
use crate::utils::Solver;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Solver.solve()?;
    day2::Solver.solve()?;
    day3::Solver.solve()?;
    day4::Solver.solve()?;
    day5::Solver.solve()?;
    day6::Solver.solve()?;
    Ok(())
}
