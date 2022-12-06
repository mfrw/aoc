use crate::utils;

pub struct Solver;

impl utils::Solver<6> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        const WINDOW: usize = 4;
        Ok(get_marker_pos(input, WINDOW))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        const WINDOW: usize = 14;
        Ok(get_marker_pos(input, WINDOW))
    }
}

fn get_marker_pos(input: &str, marker_len: usize) -> usize {
    input
        .as_bytes()
        .windows(marker_len)
        .position(|w| all_uniq(w))
        .unwrap()
        + marker_len
}

// pretty dumb; but find a better way latter
fn all_uniq<T>(i: &[T]) -> bool
where
    T: std::hash::Hash + std::cmp::Eq,
{
    use std::collections::HashSet;
    let st: HashSet<&T> = HashSet::from_iter(i.into_iter());
    st.len() == i.len()
}
