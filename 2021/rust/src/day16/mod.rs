use crate::utils;

enum Instruction {
    Literal(u8, usize),
    Operator(u8, u8, Vec<Instruction>),
}

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    let ins = parse_instruction(&input, 1);
    println!("Day16/Part1 Sol: {}", part1(&ins.1[0]));
    Ok(())
}

fn part1(input: &Instruction) -> usize {
    version_sum(input)
}

fn get_input() -> Result<Vec<u8>, std::io::Error> {
    let input = utils::get_input("input/day16")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut v = vec![];
    input
        .trim_matches('\n')
        .chars()
        .map(|c| {
            let bits = match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => unreachable!(),
            };
            bits.bytes().map(|b| b - b'0').collect::<Vec<u8>>()
        })
        .for_each(|b| v.extend(b));
    Ok(v)
}

fn from_bits(bits: &[u8]) -> usize {
    let mut x = 0;
    for &b in bits {
        x = (x << 1) | b as usize
    }
    x
}

fn parse_instruction(b: &[u8], max_insts: usize) -> (usize, Vec<Instruction>) {
    let mut i = 0;
    let mut insts = Vec::new();
    while i < b.len() && insts.len() < max_insts {
        let version = from_bits(&b[i..(i + 3)]) as u8;
        let type_id = from_bits(&b[(i + 3)..(i + 6)]);
        i += 6;
        match type_id {
            4 => {
                let mut val = 0;
                loop {
                    val <<= 4;
                    let x = from_bits(&b[i..(i + 5)]);
                    val += x & 0xf;
                    i += 5;
                    if x >> 4 == 0 {
                        break;
                    }
                }
                insts.push(Instruction::Literal(version, val));
            }
            id => {
                i += 1;
                let new_insts = match b[i - 1] {
                    0 => {
                        let nbits = from_bits(&b[i..(i + 15)]) as usize;
                        i += 15;
                        let (_, new_insts) = parse_instruction(&b[i..(i + nbits)], usize::MAX);
                        i += nbits;
                        new_insts
                    }
                    _ => {
                        let ninsts = from_bits(&b[i..(i + 11)]) as usize;
                        i += 11;
                        let (j, new_insts) = parse_instruction(&b[i..], ninsts);
                        i += j;
                        new_insts
                    }
                };
                insts.push(Instruction::Operator(version, id as u8, new_insts));
            }
        }
    }
    (i, insts)
}

fn version_sum(inst: &Instruction) -> usize {
    match inst {
        Instruction::Literal(version, _) => *version as usize,
        Instruction::Operator(version, _, insts) => {
            *version as usize + insts.iter().map(version_sum).sum::<usize>()
        }
    }
}
