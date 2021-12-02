use crate::utils;
use std::error::Error;

enum DriveCmd {
    Forward(i32),
    Down(i32),
    Up(i32),
    Unknown,
}

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let start1 = Position { x: 0, y: 0, aim: 0 };
    let end1 = part1(start1, &input)?;
    println!("Day2/Part1 Sol: {}", end1.x * end1.y);
    let start2 = Position { x: 0, y: 0, aim: 0 };
    let end2 = part2(start2, &input)?;
    println!("Day2/Part2 Sol: {}", end2.x * end2.y);
    Ok(())
}

fn part2(start: Position, cmds: &[DriveCmd]) -> Result<Position, Box<dyn Error>> {
    use DriveCmd::*;
    let mut end = Position { ..start };
    for i in cmds {
        match i {
            Forward(x) => {
                end.x += x;
                end.y += end.aim * x;
            }
            Down(y) => {
                end.aim += y;
            }
            Up(y) => {
                end.aim -= y;
            }
            Unknown => {}
        }
    }
    Ok(end)
}

fn part1(start: Position, cmds: &[DriveCmd]) -> Result<Position, Box<dyn Error>> {
    use DriveCmd::*;
    let mut end = Position { ..start };
    for i in cmds {
        match i {
            Forward(x) => end.x += x,
            Down(y) => end.y += y,
            Up(y) => end.y -= y,
            Unknown => {}
        }
    }
    Ok(end)
}

fn get_input() -> Result<Vec<DriveCmd>, Box<dyn Error>> {
    let input = utils::get_input("input/day2")?;

    let mut vec: Vec<DriveCmd> = vec![];

    for i in input.lines() {
        let parts: Vec<_> = i.split_whitespace().collect();
        let val: i32 = parts[1].parse()?;
        let cmd = match parts[0] {
            "forward" => DriveCmd::Forward(val),
            "up" => DriveCmd::Up(val),
            "down" => DriveCmd::Down(val),
            _ => DriveCmd::Unknown,
        };
        vec.push(cmd);
    }
    Ok(vec)
}
