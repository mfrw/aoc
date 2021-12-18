use core::fmt;
use std::ops::Add;

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day18/Part1 Sol: {}", part1(&input));
    println!("Day18/Part2 Sol: {}", part2(&input));

    Ok(())
}

fn part1(input: &[String]) -> i64 {
    input
        .iter()
        .map(|x| Comm::new(x.to_string()))
        .fold(Comm::None, |tot, x| tot + x)
        .magnitude()
}

fn part2(input: &[String]) -> i64 {
    let data = input
        .iter()
        .map(|x| Comm::new(x.to_string()))
        .collect::<Vec<_>>();
    (0..data.len())
        .map(|i| {
            (0..data.len())
                .filter(|&j| j != i)
                .map(|j| (data[i].clone() + data[j].clone()).magnitude())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn get_input() -> Result<Vec<String>, std::io::Error> {
    let input = utils::get_input("input/day18")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<String>, std::io::Error> {
    let v = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    Ok(v)
}

#[derive(Clone)]
enum Comm {
    A(Box<Comm>, Box<Comm>),
    B(i64),
    None,
}

impl Comm {
    fn new(inp: String) -> Self {
        if let Some(a) = inp.parse::<i64>().ok() {
            Comm::B(a)
        } else {
            let mut position = 0;
            let mut open = 0;
            for (i, ch) in inp.chars().enumerate() {
                match ch {
                    '[' => open += 1,
                    ']' => open -= 1,
                    ',' if open == 1 => position = i,
                    _ => {}
                }
            }
            Comm::new(inp[1..position].to_owned())
                + Comm::new(inp[position + 1..inp.chars().count() - 1].to_owned())
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Comm::A(a, b) => {
                if !a.split() {
                    b.split()
                } else {
                    true
                }
            }
            Comm::B(a) => {
                if *a >= 10 {
                    *self = match std::mem::replace(self, Comm::None) {
                        Comm::B(a) => {
                            Comm::A(Box::new(Comm::B(a / 2)), Box::new(Comm::B(a / 2 + a % 2)))
                        }
                        v => v,
                    };
                    true
                } else {
                    false
                }
            }
            Comm::None => unreachable!(),
        }
    }

    fn adder(&mut self, from: bool, with: i64) {
        match self {
            Comm::A(a, b) => {
                if from {
                    b.adder(from, with);
                } else {
                    a.adder(from, with);
                }
            }
            Comm::B(a) => *a += with,
            Comm::None => panic!(),
        }
    }

    fn pangs(&mut self, depth: usize) -> Option<(i64, i64)> {
        match self {
            Comm::A(a, b) => {
                if depth == 4 {
                    let left = a.pangs(depth + 1).unwrap().0;
                    let right = b.pangs(depth + 1).unwrap().0;
                    *self = match std::mem::replace(self, Comm::None) {
                        Comm::A(_, _) => Comm::B(0),
                        v => v,
                    };
                    Some((left, right))
                } else {
                    if let Some(a) = a.pangs(depth + 1) {
                        b.adder(false, a.1);
                        Some((a.0, 0))
                    } else if let Some(b) = b.pangs(depth + 1) {
                        a.adder(true, b.0);
                        Some((0, b.1))
                    } else {
                        None
                    }
                }
            }
            Comm::B(a) => {
                if depth > 4 {
                    Some((*a, *a))
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Comm::A(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            Comm::B(a) => *a,
            Comm::None => unreachable!(),
        }
    }
}

impl Add for Comm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ret = match self {
            Comm::None => other,
            a => Comm::A(Box::new(a), Box::new(other)),
        };
        // tried turning this into a normal while loop. Didn't work. So have this monstrosity.
        while {
            while let Some(_) = ret.pangs(0) {}
            ret.split()
        } {}
        ret
    }
}

impl fmt::Debug for Comm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comm::A(a, b) => {
                write!(f, "[")?;
                write!(f, "{:?}", a)?;
                write!(f, ", ")?;
                write!(f, "{:?}", b)?;
                write!(f, "]")
            }
            Comm::B(a) => {
                write!(f, "{:?}", a)
            }
            Comm::None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(4173, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = get_input().unwrap();
        assert_eq!(4706, part2(&input));
    }
}
