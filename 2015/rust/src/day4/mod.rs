use crate::utils;
use md5;
use rayon::prelude::*;

pub struct Day;

impl utils::Solver<4> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let i = i.trim();
        let pat = "00000";
        // tweaked to keep runtime sane
        let ans = (200000..1000000)
            .into_par_iter()
            .find_first(|nr| compute_md5(&(i.to_string() + &nr.to_string()), pat))
            .unwrap();
        Ok(ans)
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let i = i.trim();
        let pat = "000000";
        // tweaked to keep runtime sane
        let ans = (9900000..10000000)
            .into_par_iter()
            .find_first(|nr| compute_md5(&(i.to_string() + &nr.to_string()), pat))
            .unwrap();
        Ok(ans)
    }
}

fn compute_md5(i: &str, pat: &str) -> bool {
    let d = format!("{:?}", md5::compute(i.as_bytes()));
    d.starts_with(pat)
}
