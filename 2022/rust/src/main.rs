#![feature(binary_heap_into_iter_sorted)]

#[macro_use]
extern crate scan_fmt;

mod utils;

use crate::utils::Solver;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::Solver.solve()?;
    day2::Solver.solve()?;
    day3::Solver.solve()?;
    day4::Solver.solve()?;
    day5::Solver.solve()?;
    day6::Solver.solve()?;
    day7::Solver.solve()?;
    day8::Solver.solve()?;
    day9::Solver.solve()?;
    day10::Solver.solve()?;
    day11::Solver.solve()?;
    day12::Solver.solve()?;
    day13::Solver.solve()?;
    day14::Solver.solve()?;
    day15::Solver.solve()?;
    day16::Solver.solve()?;
    day17::Solver.solve()?;
    day18::Solver.solve()?;
    day19::Solver.solve()?;
    day20::Solver.solve()?;
    day21::Solver.solve()?;
    day22::Solver.solve()?;
    day23::Solver.solve()?;
    day24::Solver.solve()?;
    day25::Solver.solve()?;
    Ok(())
}
