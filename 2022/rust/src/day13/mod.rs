use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded},
    Finish, IResult,
};

use crate::utils;

pub struct Solver;

impl utils::Solver<13> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut sum = 0;
        let mut idx = 1;
        input.split("\n\n").for_each(|inp| {
            let lines = &mut inp.lines();
            while let Some(line) = lines.next() {
                let a = packet_parser::<i32>(line).unwrap();
                let b = packet_parser::<i32>(lines.next().unwrap()).unwrap();
                if a <= b {
                    sum += idx;
                }
            }
            idx += 1;
        });
        Ok(sum)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let divider1 = packet_parser::<i32>("[[2]]").unwrap();
        let divider2 = packet_parser::<i32>("[[6]]").unwrap();

        let mut all_packets = vec![divider1.clone(), divider2.clone()];
        input.split("\n\n").for_each(|inp| {
            let lines = &mut inp.lines();
            while let Some(line) = lines.next() {
                let a = packet_parser(line).unwrap();
                let b = packet_parser(lines.next().unwrap()).unwrap();
                all_packets.extend_from_slice(&[a, b]);
            }
        });
        all_packets.sort();
        let mut prod = 0;
        for (i, packet) in all_packets.into_iter().enumerate() {
            if packet == divider1 {
                prod = i + 1;
            } else if packet == divider2 {
                prod *= i + 1;
                break;
            }
        }
        Ok(prod)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Packet<T>
where
    T: Ord + Copy,
{
    list: Vec<Value<T>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value<T>
where
    T: Ord + Copy,
{
    Int(T),
    List(Vec<Value<T>>),
}

impl<T> PartialOrd for Value<T>
where
    T: std::cmp::PartialEq + Ord + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Value<T>
where
    T: Ord + Copy,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(a), Value::List(b)) => vec![Value::Int(*a)].cmp(b),
            (Value::List(a), Value::Int(b)) => a.cmp(&vec![Value::Int(*b)]),
            (Value::List(a), Value::List(b)) => a.cmp(b),
        }
    }
}

fn sp(i: &str) -> IResult<&str, &str> {
    multispace0(i)
}

fn parse_number<T: std::str::FromStr>(i: &str) -> IResult<&str, T> {
    let nr = map_res(
        take_while1(|c: char| c.is_ascii_digit() || c == '-'),
        |s: &str| s.parse::<T>(),
    );
    delimited(sp, nr, sp)(i)
}

fn parse_item_list<T>(i: &str) -> IResult<&str, Vec<Value<T>>>
where
    T: std::str::FromStr + Copy + Ord,
{
    delimited(
        preceded(sp, tag("[")),
        separated_list0(preceded(sp, tag(",")), parse_item_or_list),
        preceded(sp, tag("]")),
    )(i)
}

fn parse_item_or_list<T>(i: &str) -> IResult<&str, Value<T>>
where
    T: Copy + Ord + std::str::FromStr,
{
    alt((
        map(parse_number::<T>, Value::Int),
        map(parse_item_list::<T>, Value::List),
    ))(i)
}

fn packet_parser<T>(i: &str) -> Option<Packet<T>>
where
    T: std::str::FromStr + Copy + Ord,
{
    if let Ok((_, l)) = all_consuming(parse_item_list::<T>)(i).finish() {
        Some(Packet { list: l })
    } else {
        None
    }
}
