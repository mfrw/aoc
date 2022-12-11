use std::collections::HashMap;

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

fn parse_input(input: &str) -> Vec<Monkey<i64>> {
    input
        .split("\n\n")
        .map(|ipt| {
            let mut it = ipt.lines();
            it.next();

            let items = it
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(",")
                .map(|i| i.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let op = it
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Operation: new = old ")
                .map(|text| {
                    if text == "* old" {
                        Operation::Square
                    } else if let Some(value) = text.strip_prefix("* ") {
                        Operation::Mul(value.parse().unwrap())
                    } else if let Some(value) = text.strip_prefix("+ ") {
                        Operation::Add(value.parse().unwrap())
                    } else {
                        panic!("unknown operatation {}", text);
                    }
                })
                .unwrap();
            let test = it
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let tr = it
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let fl = it
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
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
    op: Operation,
    tr: usize,
    fl: usize,
    items: Vec<T>,
    inspect_count: usize,
}

#[derive(Debug)]
enum Operation {
    Square,
    Add(i64),
    Mul(i64),
}

impl Operation {
    fn apply(&self, input: i64) -> i64 {
        match self {
            Operation::Square => input * input,
            Operation::Mul(val) => input * val,
            Operation::Add(val) => input + val,
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
