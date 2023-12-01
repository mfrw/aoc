use crate::utils;

pub struct Solver;

impl utils::Solver<1> for Solver {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
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

fn part2_int(input: &str) -> Option<u32> {
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input.lines() {
        let mut digits = vec![];
        let chars = line.chars().collect::<Vec<_>>();
        for (i, c) in chars.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
                continue;
            }
            let s = String::from_iter(&chars[i..chars.len()]);
            for (i, n) in numbers.iter().enumerate() {
                if s.starts_with(n) {
                    digits.push(i as u32 + 1);
                }
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    Some(total)
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

#[test]
fn p2_test() {
    let i = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let ans = part2_int(i).unwrap();
    assert_eq!(281, ans)
}
