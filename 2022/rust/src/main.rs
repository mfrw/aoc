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
    use crate::utils::get_input;
    use crate::utils::Solver;

    fn solve_for_tests<const N: usize, P1, P2>(
        s: impl Solver<N, Part1 = P1, Part2 = P2>,
    ) -> Result<(P1, P2), Box<dyn std::error::Error>> {
        let input = get_input(&format!("input/day{}", N))?;
        Ok((s.part1(&input)?, s.part2(&input)?))
    }

    #[test]
    fn day1_test() {
        assert_eq!((68442, 204837), solve_for_tests(day1::Solver).unwrap());
    }

    #[test]
    fn day2_test() {
        let s = solve_for_tests(day2::Solver).unwrap();
        assert_eq!((9241, 14610), s);
    }

    #[test]
    fn day3_test() {
        let s = solve_for_tests(day3::Solver).unwrap();
        assert_eq!((7826, 2577), s);
    }

    #[test]
    fn day4_test() {
        let s = solve_for_tests(day4::Solver).unwrap();
        assert_eq!((644, 926), s);
    }

    #[test]
    fn day5_test() {
        let s = solve_for_tests(day5::Solver).unwrap();
        assert_eq!(("NTWZZWHFV".into(), "BRZGFVBTJ".into()), s);
    }

    #[test]
    fn day6_test() {
        let s = solve_for_tests(day6::Solver).unwrap();
        assert_eq!((1850, 2823), s);
    }

    #[test]
    fn day7_test() {
        let s = solve_for_tests(day7::Solver).unwrap();
        assert_eq!((1432936, 272298), s);
    }

    #[test]
    fn day8_test() {
        let s = solve_for_tests(day8::Solver).unwrap();
        assert_eq!((1785, 345168), s);
    }

    #[test]
    fn day9_test() {
        let s = solve_for_tests(day9::Solver).unwrap();
        assert_eq!((6081, 2487), s);
    }

    #[test]
    fn day10_test() {
        let s = solve_for_tests(day10::Solver).unwrap();
        assert_eq!(14240, s.0);
    }

    #[test]
    fn day11_test() {
        let s = solve_for_tests(day11::Solver).unwrap();
        assert_eq!((102399, 23641658401), s);
    }

    #[test]
    fn day12_test() {
        let s = solve_for_tests(day12::Solver).unwrap();
        assert_eq!((370, 363), s);
    }

    #[test]
    fn day13_test() {
        let s = solve_for_tests(day13::Solver).unwrap();
        assert_eq!((5208, 25792), s);
    }

    #[test]
    fn day14_test() {
        let s = solve_for_tests(day14::Solver).unwrap();
        assert_eq!((618, 26358), s);
    }

    #[test]
    fn day15_test() {
        let s = solve_for_tests(day15::Solver).unwrap();
        assert_eq!((4717631, 13197439355220), s);
    }

    #[test]
    fn day16_test() {
        let s = solve_for_tests(day16::Solver).unwrap();
        assert_eq!((1488, 2111), s);
    }

    #[test]
    fn day17_test() {
        let s = solve_for_tests(day17::Solver).unwrap();
        assert_eq!((3071, 1523615160362), s);
    }

    #[test]
    fn day18_test() {
        let s = solve_for_tests(day18::Solver).unwrap();
        assert_eq!((4500, 2558), s);
    }

    #[test]
    fn day19_test() {
        let s = solve_for_tests(day19::Solver).unwrap();
        assert_eq!((1528, 16926), s);
    }

    #[test]
    fn day20_test() {
        let s = solve_for_tests(day20::Solver).unwrap();
        assert_eq!((19070, 14773357352059), s);
    }

    #[test]
    fn day21_test() {
        let s = solve_for_tests(day21::Solver).unwrap();
        assert_eq!((124765768589550, 3059361893920), s);
    }

    #[test]
    fn day22_test() {
        let s = solve_for_tests(day22::Solver).unwrap();
        assert_eq!((3590, 86382), s);
    }

    #[test]
    fn day23_test() {
        let s = solve_for_tests(day23::Solver).unwrap();
        assert_eq!((4254, 992), s);
    }

    #[test]
    fn day24_test() {
        let s = solve_for_tests(day24::Solver).unwrap();
        assert_eq!((260, 747), s);
    }

    #[test]
    fn day25_test() {
        let s = solve_for_tests(day25::Solver).unwrap();
        assert_eq!("2=112--220-=-00=-=20", s.0);
    }
}
