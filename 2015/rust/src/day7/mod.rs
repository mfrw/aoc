use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, Not, Shl, Shr},
};

use nom::{
    branch::alt, bytes::complete::tag, character::complete as cc, combinator::map, sequence::tuple,
    IResult,
};

use crate::utils;

pub struct Day;

impl utils::Solver<7> for Day {
    type Part1 = u16;

    type Part2 = u16;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut sym: SymTab<u16> = SymTab::default();
        i.lines().map(|l| parse_line(l).unwrap().1).for_each(|e| {
            sym.insert(e.0, e.1);
        });
        let ans = sym.remove("a").unwrap().eval(&mut sym).unwrap();
        Ok(ans)
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut sym: SymTab<u16> = SymTab::default();
        i.lines().map(|l| parse_line(l).unwrap().1).for_each(|e| {
            if e.0 == "b" {
                sym.insert(e.0, Expr::new_term_with_value(956));
            } else {
                sym.insert(e.0, e.1);
            }
        });
        let ans = sym.remove("a").unwrap().eval(&mut sym).unwrap();
        Ok(ans)
    }
}

#[derive(Debug)]
enum Operand<T> {
    Value(T),
    Label(String),
}

#[derive(Debug)]
enum Expr<T> {
    Terminal(Operand<T>),
    And { l: Operand<T>, r: Operand<T> },
    Or { l: Operand<T>, r: Operand<T> },
    Lshift { l: Operand<T>, nr: usize },
    Rshift { l: Operand<T>, nr: usize },
    Not { l: Operand<T> },
}

#[derive(Default)]
struct SymTab<T> {
    mp: HashMap<String, Expr<T>>,
}

impl<T> SymTab<T> {
    fn insert(&mut self, k: String, v: Expr<T>) {
        self.mp.insert(k.into(), v);
    }

    fn remove(&mut self, k: &str) -> Option<Expr<T>> {
        self.mp.remove(k)
    }
}

impl<T> Operand<T>
where
    T: Clone + std::fmt::Debug + Copy,
    T: BitAnd<Output = T>,
    T: BitOr<Output = T>,
    T: Shl<usize, Output = T>,
    T: Shr<usize, Output = T>,
    T: Not<Output = T>,
{
    fn eval(&self, sym: &mut SymTab<T>) -> Option<T> {
        match self {
            Operand::Value(t) => Some(t.clone()),
            Operand::Label(l) => {
                let ans = sym.remove(l)?.eval(sym)?;
                sym.insert(l.into(), Expr::new_term_with_value(ans));
                Some(ans)
            }
        }
    }

    fn new_val(e: T) -> Self {
        Operand::Value(e)
    }

    fn new_label(s: &str) -> Self {
        Operand::Label(s.into())
    }
}

impl<T> Expr<T>
where
    T: Clone + std::fmt::Debug + Copy,
    T: BitAnd<Output = T>,
    T: BitOr<Output = T>,
    T: Shl<usize, Output = T>,
    T: Shr<usize, Output = T>,
    T: Not<Output = T>,
{
    fn eval(&self, sym: &mut SymTab<T>) -> Option<T> {
        let val = match self {
            Expr::Terminal(t) => t.eval(sym),
            Expr::And { l, r } => {
                let left = l.eval(sym)?;
                let right = r.eval(sym)?;
                Some(left.bitand(right))
            }
            Expr::Or { l, r } => {
                let left = l.eval(sym)?;
                let right = r.eval(sym)?;
                Some(left.bitor(right))
            }
            Expr::Lshift { l, nr } => {
                let left = l.eval(sym)?;
                Some(left.shl(*nr))
            }
            Expr::Rshift { l, nr } => {
                let left = l.eval(sym)?;
                Some(left.shr(*nr))
            }

            Expr::Not { l } => {
                let left = l.eval(sym)?;
                Some(left.not())
            }
        };
        val
    }
    fn new_term_with_value(v: T) -> Self {
        Expr::Terminal(Operand::Value(v))
    }
}

fn parse_lvalue(i: &str) -> IResult<&str, Operand<u16>> {
    map(cc::alpha1, Operand::new_label)(i)
}

fn parse_u16(i: &str) -> IResult<&str, Operand<u16>> {
    let (ipt, o) = cc::u16(i)?;
    Ok((ipt, Operand::new_val(o)))
}

fn parse_rvalue(i: &str) -> IResult<&str, Operand<u16>> {
    alt((parse_u16, parse_lvalue))(i)
}

fn parse_terminal(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    let (ipt, opt) = tuple((parse_rvalue, tag(" -> "), parse_lvalue))(i)?;
    let l = match opt.2 {
        Operand::Value(_) => panic!("lvalue cant be a constant"),
        Operand::Label(l) => l,
    };
    let e = Expr::Terminal(opt.0);

    Ok((ipt, (l, e)))
}

fn parse_and(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    let (ipt, opt) = tuple((
        parse_rvalue,
        tag(" AND "),
        parse_rvalue,
        tag(" -> "),
        parse_lvalue,
    ))(i)?;
    let l = match opt.4 {
        Operand::Value(_) => panic!("lvalue cant be a constant"),
        Operand::Label(l) => l,
    };
    let e = Expr::And { l: opt.0, r: opt.2 };

    Ok((ipt, (l, e)))
}

fn parse_or(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    let (ipt, opt) = tuple((
        parse_rvalue,
        tag(" OR "),
        parse_rvalue,
        tag(" -> "),
        parse_lvalue,
    ))(i)?;
    let l = match opt.4 {
        Operand::Value(_) => panic!("lvalue cant be a constant"),
        Operand::Label(l) => l,
    };
    let e = Expr::Or { l: opt.0, r: opt.2 };

    Ok((ipt, (l, e)))
}

fn parse_lsh(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    let (ipt, opt) = tuple((
        parse_lvalue,
        tag(" LSHIFT "),
        cc::u16,
        tag(" -> "),
        parse_lvalue,
    ))(i)?;
    let l = match opt.4 {
        Operand::Value(_) => panic!("lvalue cant be a constant"),
        Operand::Label(l) => l,
    };
    let e = Expr::Lshift {
        l: opt.0,
        nr: opt.2 as usize,
    };

    Ok((ipt, (l, e)))
}

fn parse_rsh(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    let (ipt, opt) = tuple((
        parse_lvalue,
        tag(" RSHIFT "),
        cc::u16,
        tag(" -> "),
        parse_lvalue,
    ))(i)?;
    let l = match opt.4 {
        Operand::Value(_) => panic!("lvalue cant be a constant"),
        Operand::Label(l) => l,
    };
    let e = Expr::Rshift {
        l: opt.0,
        nr: opt.2 as usize,
    };

    Ok((ipt, (l, e)))
}

fn parse_not(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    let (ipt, opt) = tuple((tag("NOT "), parse_lvalue, tag(" -> "), parse_lvalue))(i)?;
    let l = match opt.3 {
        Operand::Value(_) => panic!("lvalue cant be a constant"),
        Operand::Label(l) => l,
    };
    let e = Expr::Not { l: opt.1 };

    Ok((ipt, (l, e)))
}

fn parse_line(i: &str) -> IResult<&str, (String, Expr<u16>)> {
    alt((
        parse_terminal,
        parse_and,
        parse_or,
        parse_lsh,
        parse_rsh,
        parse_not,
    ))(i)
}

#[test]
fn basic_dep_eval() {
    let mut sym: SymTab<u16> = SymTab::default();
    sym.insert("x".into(), Expr::Terminal(Operand::Value(123)));
    sym.insert("y".into(), Expr::Terminal(Operand::Value(456)));
    sym.insert(
        "d".into(),
        Expr::And {
            l: Operand::new_label("x"),
            r: Operand::new_label("y"),
        },
    );
    sym.insert(
        "e".into(),
        Expr::Or {
            l: Operand::new_label("x"),
            r: Operand::new_label("y"),
        },
    );

    sym.insert(
        "f".into(),
        Expr::Lshift {
            l: Operand::new_label("x"),
            nr: 2,
        },
    );
    sym.insert(
        "g".into(),
        Expr::Rshift {
            l: Operand::new_label("y"),
            nr: 2,
        },
    );
    sym.insert(
        "h".into(),
        Expr::Not {
            l: Operand::new_label("x"),
        },
    );
    sym.insert(
        "i".into(),
        Expr::Not {
            l: Operand::new_label("y"),
        },
    );
    assert_eq!(sym.remove("e").unwrap().eval(&mut sym), Some(507));
    assert_eq!(sym.remove("f").unwrap().eval(&mut sym), Some(492));
    assert_eq!(sym.remove("h").unwrap().eval(&mut sym), Some(65412));
}
