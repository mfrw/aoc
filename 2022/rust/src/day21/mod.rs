use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map, value};
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

use crate::utils;

pub struct Solver;

impl utils::Solver<21> for Solver {
    type Part1 = i128;

    type Part2 = i128;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut mp = AllMonkeys::new();
        input
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .for_each(|m| mp.add_monkey(m));
        let ans = mp.eval("root");
        Ok(ans)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}

struct AllMonkeys {
    mp: HashMap<String, Monkey>,
}

impl AllMonkeys {
    fn new() -> Self {
        Self { mp: HashMap::new() }
    }

    fn add_monkey(&mut self, m: Monkey) {
        self.mp.entry(m.name.clone()).or_insert(m);
    }

    fn eval(&self, monkey: &str) -> i128 {
        let monkey = self.mp.get(monkey).unwrap();

        let v = match &monkey.dep {
            Dep::Terminal(n) => *n,
            Dep::Expr { l, r, op } => {
                let left = self.eval(&l);
                let right = self.eval(&r);
                match &op {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Div => left / right,
                    Op::Mul => left * right,
                }
            }
        };
        v
    }
}

#[derive(Clone)]
struct Monkey {
    name: String,
    dep: Dep,
}

#[derive(Clone)]
enum Dep {
    Terminal(i128),
    Expr { l: String, r: String, op: Op },
}

#[derive(Clone)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

fn parse_single_term(i: &str) -> IResult<&str, Dep> {
    map(cc::i128, Dep::Terminal)(i)
}

fn parse_multiple_terms(i: &str) -> IResult<&str, Dep> {
    let (i, o) = tuple((cc::alpha1, parse_bin_op, cc::alpha1))(i)?;
    Ok((
        i,
        Dep::Expr {
            l: o.0.to_string(),
            r: o.2.to_string(),
            op: o.1,
        },
    ))
}

fn parse_bin_op(i: &str) -> IResult<&str, Op> {
    alt((
        value(Op::Add, tag(" + ")),
        value(Op::Sub, tag(" - ")),
        value(Op::Div, tag(" / ")),
        value(Op::Mul, tag(" * ")),
    ))(i)
}

fn parse_line(i: &str) -> IResult<&str, Monkey> {
    let (i, o) = tuple((
        cc::alpha1,
        tag(": "),
        alt((parse_single_term, parse_multiple_terms)),
    ))(i)?;
    Ok((
        i,
        Monkey {
            name: o.0.to_string(),
            dep: o.2,
        },
    ))
}
