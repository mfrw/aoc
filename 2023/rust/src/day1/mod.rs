use crate::utils;

pub struct Solver;

impl utils::Solver<1> for Solver {
    type Part1 = u32;
    type Part2 = i32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input))
    }
}

fn part1_int(input: &str) -> Option<u32> {
    let mut ans = 0;
    for l in input.lines() {
        let v = l
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        if v.is_empty() {
            continue;
        }
        let ln = v.len();
        let tmpans = (10 * v[0]) + v[ln - 1];
        ans += tmpans;
    }
    Some(ans)
}

fn part2_int(input: &str) -> i32 {
    unreachable!()
}

#[test]
fn p1_test() {
    let i = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let ans = part1_int(i).unwrap();
    assert_eq!(142, ans)
}
