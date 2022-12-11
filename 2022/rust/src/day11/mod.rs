use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::utils;

pub struct Solver;

impl utils::Solver<11> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(solve1(input))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(solve2(input))
    }
}

fn last_word(l: &str) -> Option<&str> {
    l.split_whitespace().last()
}

fn last_item<T>(l: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let i = last_word(l).unwrap();
    T::from_str(i).unwrap()
}

fn parse_input<T>(input: &str) -> Vec<Monkey<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split("\n\n")
        .map(|ipt| {
            let input = ipt.lines().collect::<Vec<_>>();

            let items = input[1]
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(",")
                .map(|i| i.trim().parse::<T>().unwrap())
                .collect::<Vec<_>>();

            let op = input[2].trim().parse::<Operation<_>>().unwrap();
            let test = last_item(input[3]);
            let tr = last_item(input[4]);
            let fl = last_item(input[5]);

            Monkey {
                op,
                test,
                tr,
                fl,
                items,
                inspect_count: 0,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Monkey<T> {
    test: T,
    op: Operation<T>,
    tr: usize,
    fl: usize,
    items: Vec<T>,
    inspect_count: usize,
}

#[derive(Debug)]
enum Operation<T> {
    Square,
    Add(T),
    Mul(T),
}

impl<T> FromStr for Operation<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = s
            .trim()
            .strip_prefix("Operation: new = old ")
            .map(|text| {
                if text == "* old" {
                    Ok(Operation::Square)
                } else if let Some(value) = text.strip_prefix("* ") {
                    Ok(Operation::Mul(value.parse().unwrap()))
                } else if let Some(value) = text.strip_prefix("+ ") {
                    Ok(Operation::Add(value.parse().unwrap()))
                } else {
                    Err(format!("unknown operatation {}", text))
                }
            })
            .unwrap();
        op
    }
}

impl<T> Operation<T>
where
    T: Copy,
    T: std::ops::Add<T, Output = T>,
    T: std::ops::Mul<T, Output = T>,
{
    fn apply(&self, input: T) -> T {
        match self {
            Operation::Square => input * input,
            Operation::Mul(val) => input * val.clone(),
            Operation::Add(val) => input + val.clone(),
        }
    }
}

fn solve1(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    let adjust = |i: i64| -> i64 { i / 3 };
    for _ in 0..20 {
        do_round(&mut monkeys, adjust);
    }
    monkeys.sort_by_cached_key(|monkey| monkey.inspect_count);
    monkeys[monkeys.len() - 1].inspect_count * monkeys[monkeys.len() - 2].inspect_count
}

fn solve2(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    let modulo: i64 = monkeys.iter().map(|monkey| monkey.test).product();
    let adjust = |i: i64| -> i64 { i % modulo };
    for _ in 0..10_000 {
        do_round(&mut monkeys, adjust);
    }
    monkeys.sort_by_cached_key(|monkey| monkey.inspect_count);
    monkeys[monkeys.len() - 1].inspect_count * monkeys[monkeys.len() - 2].inspect_count
}

fn do_round(monkeys: &mut [Monkey<i64>], adjust_worry_fn: impl Fn(i64) -> i64) {
    for id in 0..monkeys.len() {
        let mut targets = HashMap::<_, Vec<_>>::new();
        {
            let monkey = &mut monkeys[id];
            monkey.inspect_count += monkey.items.len();

            for item in monkey.items.drain(..) {
                let new_worry_level = adjust_worry_fn(monkey.op.apply(item));

                if new_worry_level % monkey.test == 0 {
                    targets.entry(monkey.tr).or_default().push(new_worry_level);
                } else {
                    targets.entry(monkey.fl).or_default().push(new_worry_level);
                }
            }
        }

        for (id, items) in targets {
            monkeys[id].items.extend(items.into_iter());
        }
    }
}
