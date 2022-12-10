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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        let s = day1::Solver.solve_internal().unwrap();
        assert_eq!((68442, 204837), s);
    }

    #[test]
    fn day2_test() {
        let s = day2::Solver.solve_internal().unwrap();
        assert_eq!((9241, 14610), s);
    }

    #[test]
    fn day3_test() {
        let s = day3::Solver.solve_internal().unwrap();
        assert_eq!((7826, 2577), s);
    }

    #[test]
    fn day4_test() {
        let s = day4::Solver.solve_internal().unwrap();
        assert_eq!((644, 926), s);
    }

    #[test]
    fn day5_test() {
        let s = day5::Solver.solve_internal().unwrap();
        assert_eq!(("NTWZZWHFV".into(), "BRZGFVBTJ".into()), s);
    }

    #[test]
    fn day6_test() {
        let s = day6::Solver.solve_internal().unwrap();
        assert_eq!((1850, 2823), s);
    }

    #[test]
    fn day7_test() {
        let s = day7::Solver.solve_internal().unwrap();
        assert_eq!((1432936, 272298), s);
    }

    #[test]
    fn day8_test() {
        let s = day8::Solver.solve_internal().unwrap();
        assert_eq!((1785, 345168), s);
    }

    #[test]
    fn day9_test() {
        let s = day9::Solver.solve_internal().unwrap();
        assert_eq!((6081, 2487), s);
    }

    #[test]
    fn day10_test() {
        let s = day10::Solver.solve_internal().unwrap();
        assert_eq!(14240, s.0);
    }
}
