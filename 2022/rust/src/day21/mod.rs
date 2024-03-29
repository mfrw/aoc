use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    combinator::{all_consuming, map, value},
    sequence::tuple,
    Finish, IResult,
};

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
        let mut mp = AllMonkeys::new();
        input
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .for_each(|m| mp.add_monkey(m));

        let old = mp.eval("hsdb").cmp(&mp.eval("mwrd"));

        let answer = binary_search(i64::MIN as i128, i64::MAX as i128, |x| {
            mp.clear_lookaside();
            mp.mp
                .entry("humn".into())
                .and_modify(|v| v.dep = Dep::Terminal(x));

            let new = mp.eval("hsdb").cmp(&mp.eval("mwrd"));
            match new {
                Ordering::Equal => Ordering::Equal,
                new => {
                    if old == new {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
        })
        .unwrap();
        Ok(answer)
    }
}

fn binary_search<F>(mut lo: i128, mut hi: i128, mut cmp: F) -> Option<i128>
where
    F: FnMut(i128) -> Ordering,
{
    while lo != hi {
        let mid = (lo + hi) / 2;
        match cmp(mid) {
            // Always return the leftmost result
            Ordering::Equal => hi = mid,
            Ordering::Less => hi = mid - 1,
            Ordering::Greater => lo = mid + 1,
        }
    }
    match cmp(lo) {
        Ordering::Equal => Some(lo),
        Ordering::Less => None,
        Ordering::Greater => None,
    }
}

struct AllMonkeys<'a> {
    mp: HashMap<&'a str, Monkey<'a>>,
    lookaside: Rc<RefCell<HashMap<&'a str, i128>>>,
}

impl<'a> AllMonkeys<'a> {
    fn new() -> Self {
        Self {
            mp: HashMap::new(),
            lookaside: Default::default(),
        }
    }

    fn add_monkey(&mut self, m: Monkey<'a>) {
        self.mp.entry(m.name.clone()).or_insert(m);
    }

    fn eval(&self, monkey_name: &'a str) -> i128 {
        if self.lookaside.borrow().contains_key(monkey_name) {
            return *self.lookaside.borrow().get(monkey_name).unwrap();
        }

        let monkey = self.mp.get(monkey_name).unwrap();

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
        self.lookaside.borrow_mut().insert(monkey_name, v);
        v
    }

    fn clear_lookaside(&self) {
        self.lookaside.borrow_mut().clear();
    }
}

#[derive(Clone)]
struct Monkey<'a> {
    name: &'a str,
    dep: Dep<'a>,
}

#[derive(Clone)]
enum Dep<'a> {
    Terminal(i128),
    Expr { l: &'a str, r: &'a str, op: Op },
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

fn parse_multiple_terms<'a>(i: &'a str) -> IResult<&str, Dep<'a>> {
    let (i, o) = tuple((cc::alpha1, parse_bin_op, cc::alpha1))(i)?;
    Ok((
        i,
        Dep::Expr {
            l: o.0,
            r: o.2,
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

fn parse_line<'a>(i: &'a str) -> IResult<&str, Monkey<'a>> {
    let (i, o) = tuple((
        cc::alpha1,
        tag(": "),
        alt((parse_single_term, parse_multiple_terms)),
    ))(i)?;
    Ok((
        i,
        Monkey {
            name: o.0,
            dep: o.2,
        },
    ))
}
