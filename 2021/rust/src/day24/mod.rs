use std::str::FromStr;

use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let program = get_input()?;
    println!("Day24/Part1 Sol: {}", part1(&program).unwrap());
    println!("Day24/Part2 Sol: {}", part2(&program).unwrap());
    Ok(())
}

fn convert_to_vec_i32(n: i64) -> Option<Vec<i32>> {
    let s = n.to_string();
    let v = s
        .chars()
        .map(|ch| ch.to_string().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if v.contains(&0) {
        None
    } else {
        Some(v)
    }
}

fn part2(program: &[Ins]) -> Option<i64> {
    let mut alu = ALU::new();
    let i = 48111514719111;
    let input = convert_to_vec_i32(i).unwrap();
    alu.run(program, &input).unwrap();
    if alu.z() == &0 {
        Some(i)
    } else {
        None
    }
}

fn part1(program: &[Ins]) -> Option<i64> {
    let mut alu = ALU::new();
    let i = 99995969919326;
    let input = convert_to_vec_i32(i).unwrap();
    alu.run(program, &input).unwrap();
    if alu.z() == &0 {
        Some(i)
    } else {
        None
    }
}

fn get_input() -> Result<Vec<Ins>, std::io::Error> {
    let input = utils::get_input("input/day24")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Ins>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| Ins::from_str(l).unwrap())
        .collect::<Vec<_>>();
    Ok(v)
}

enum Register {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Register {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = match s {
            "x" => Register::X,
            "y" => Register::Y,
            "w" => Register::W,
            "z" => Register::Z,
            _ => panic!("Got {}", s),
        };
        Ok(reg)
    }
}

enum Operand {
    Var(Register),
    Imm(i32),
}

impl FromStr for Operand {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "x" => Operand::Var(Register::X),
            "w" => Operand::Var(Register::W),
            "y" => Operand::Var(Register::Y),
            "z" => Operand::Var(Register::Z),
            _ => {
                let im = s.parse::<i32>()?;
                Operand::Imm(im)
            }
        };
        Ok(op)
    }
}

enum Ins {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

impl FromStr for Ins {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        let ins = match parts[0] {
            "inp" => {
                let reg = Register::from_str(parts[1])?;
                Ins::Inp(reg)
            }
            "add" => {
                let op1 = Register::from_str(parts[1])?;
                let op2 = Operand::from_str(parts[2])?;
                Ins::Add(op1, op2)
            }
            "mul" => {
                let op1 = Register::from_str(parts[1])?;
                let op2 = Operand::from_str(parts[2])?;
                Ins::Mul(op1, op2)
            }
            "div" => {
                let op1 = Register::from_str(parts[1])?;
                let op2 = Operand::from_str(parts[2])?;
                Ins::Div(op1, op2)
            }
            "mod" => {
                let op1 = Register::from_str(parts[1])?;
                let op2 = Operand::from_str(parts[2])?;
                Ins::Mod(op1, op2)
            }
            "eql" => {
                let op1 = Register::from_str(parts[1])?;
                let op2 = Operand::from_str(parts[2])?;
                Ins::Eql(op1, op2)
            }
            _ => panic!("Got {}", parts[0]),
        };
        Ok(ins)
    }
}

struct ALU {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    pos: usize,
}

impl ALU {
    fn new() -> Self {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            pos: 0,
        }
    }

    fn run(&mut self, instructions: &[Ins], input: &[i32]) -> Result<(), std::io::Error> {
        if self.pos == input.len() {
            return Ok(());
        }

        for ins in instructions {
            match ins {
                Ins::Inp(r) => {
                    let val = input[self.pos];
                    self.pos += 1;
                    self.input(r, &val)?;
                }
                Ins::Add(a, b) => {
                    self.binop(a, b, "+")?;
                }
                Ins::Mul(a, b) => {
                    self.binop(a, b, "*")?;
                }
                Ins::Div(a, b) => {
                    self.binop(a, b, "/")?;
                }
                Ins::Mod(a, b) => {
                    self.binop(a, b, "%")?;
                }
                Ins::Eql(a, b) => {
                    self.binop(a, b, "==")?;
                }
            }
        }
        Ok(())
    }

    fn binop(&mut self, a: &Register, b: &Operand, op: &str) -> Result<(), std::io::Error> {
        let op2 = match b {
            Operand::Imm(a) => *a,
            Operand::Var(r) => match r {
                Register::W => self.w,
                Register::X => self.x,
                Register::Y => self.y,
                Register::Z => self.z,
            },
        };

        let input = match a {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        };

        match op {
            "+" => {
                *input = *input + op2;
                Ok(())
            }
            "-" => {
                *input = *input - op2;
                Ok(())
            }
            "*" => {
                *input = *input * op2;
                Ok(())
            }
            "%" => {
                *input = *input % op2;
                Ok(())
            }
            "==" => {
                if *input == op2 {
                    *input = 1;
                } else {
                    *input = 0;
                }
                Ok(())
            }
            "/" => {
                if op2 == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Op2==0",
                    ));
                }
                *input = *input / op2;
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn input(&mut self, reg: &Register, val: &i32) -> Result<(), std::io::Error> {
        match reg {
            Register::W => self.w = *val,
            Register::X => self.x = *val,
            Register::Y => self.y = *val,
            Register::Z => self.z = *val,
        }
        Ok(())
    }

    fn z(&self) -> &i32 {
        &self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let pgm = get_input().unwrap();
        assert_eq!(99995969919326, part1(&pgm).unwrap());
    }
    #[test]
    fn part2_test() {
        let pgm = get_input().unwrap();
        assert_eq!(48111514719111, part2(&pgm).unwrap());
    }
}
