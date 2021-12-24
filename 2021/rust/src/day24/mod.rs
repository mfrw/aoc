use crate::utils;

use std::collections::HashMap;

pub fn main() -> Result<(), std::io::Error> {
    let program = get_input()?;
    println!("Day24/Part1 Sol: {}", part1(&program));
    //println!("Day24/Part2 Sol: {}", part2(&program));
    Ok(())
}

fn part1(program: &[Instruction]) -> i64 {
    solve(program, false)
}

#[allow(dead_code)]
fn part2(program: &[Instruction]) -> i64 {
    solve(program, true)
}

enum RegOrVal {
    Reg(usize),
    Val(i64),
}

impl RegOrVal {
    fn to_val(&self, reg: &[i64; 4]) -> i64 {
        match self {
            RegOrVal::Reg(i) => reg[*i],
            RegOrVal::Val(v) => *v,
        }
    }
}

enum Instruction {
    Inp(usize),
    Add(usize, RegOrVal),
    Mul(usize, RegOrVal),
    Div(usize, RegOrVal),
    Mod(usize, RegOrVal),
    Eql(usize, RegOrVal),
}

impl Instruction {
    fn exec(&self, reg: &mut [i64; 4], input: Option<i64>) {
        match self {
            Instruction::Inp(a) => {
                reg[*a] = input.unwrap();
            }
            Instruction::Add(a, b) => {
                reg[*a] += b.to_val(reg);
            }
            Instruction::Mul(a, b) => {
                reg[*a] *= b.to_val(reg);
            }
            Instruction::Div(a, b) => {
                reg[*a] /= b.to_val(reg);
            }
            Instruction::Mod(a, b) => {
                reg[*a] %= b.to_val(reg);
            }
            Instruction::Eql(a, b) => {
                reg[*a] = if reg[*a] == b.to_val(reg) { 1 } else { 0 };
            }
        }
    }
}

fn reg_idx(reg: char) -> usize {
    match reg {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => panic!(),
    }
}

fn best(
    program: &[Instruction],
    pc: usize,
    reg: [i64; 4],
    visited: &mut HashMap<([i64; 4], usize), Option<i64>>,
    smallest: bool,
) -> Option<i64> {
    assert!(matches!(program[pc], Instruction::Inp(_)));

    if let Some(answer) = visited.get(&(reg, pc)) {
        return *answer;
    }

    let range = if smallest {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    } else {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    };
    'inputs: for input in range {
        let mut reg = reg;
        let mut pc = pc;
        program[pc].exec(&mut reg, Some(input));
        pc += 1;

        while let Some(inst) = program.get(pc) {
            if matches!(program[pc], Instruction::Inp(_)) {
                if let Some(best) = best(program, pc, reg, visited, smallest) {
                    visited.insert((reg, pc), Some(best * 10 + input));
                    return Some(best * 10 + input);
                } else {
                    continue 'inputs;
                }
            } else {
                inst.exec(&mut reg, None);
                pc += 1;
            }
        }

        if reg[3] == 0 {
            visited.insert((reg, pc), Some(input));
            return Some(input);
        }
    }

    visited.insert((reg, pc), None);
    None
}

fn solve(program: &[Instruction], smallest: bool) -> i64 {
    let mut memo = HashMap::new();
    let answer = best(program, 0, [0; 4], &mut memo, smallest);
    format!("{}", answer.unwrap())
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn get_input() -> Result<Vec<Instruction>, std::io::Error> {
    let input = utils::get_input("input/day24")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, std::io::Error> {
    let mut program = Vec::new();

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let instruction = parts.next().unwrap();
        let reg: char = parts.next().unwrap().parse().unwrap();
        let b_value = parts.next().map(|a| {
            a.parse::<i64>()
                .map(RegOrVal::Val)
                .unwrap_or_else(|_| RegOrVal::Reg(reg_idx(a.parse::<char>().unwrap())))
        });

        let reg = reg_idx(reg);

        program.push(match (instruction, reg, b_value) {
            ("inp", reg, None) => Instruction::Inp(reg),
            ("add", a, Some(b)) => Instruction::Add(a, b),
            ("mul", a, Some(b)) => Instruction::Mul(a, b),
            ("div", a, Some(b)) => Instruction::Div(a, b),
            ("mod", a, Some(b)) => Instruction::Mod(a, b),
            ("eql", a, Some(b)) => Instruction::Eql(a, b),
            _ => {
                panic!();
            }
        });
    }
    Ok(program)
}
