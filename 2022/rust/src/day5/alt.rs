use crate::utils;
use std::str::FromStr;

pub struct Solver;

impl utils::Solver<5> for Solver {
    type Part1 = String;

    type Part2 = String;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut state = initial_state();
        input
            .lines()
            .skip(10)
            .map(|l| l.parse::<Move>().unwrap())
            .for_each(|m| state.process(&m));
        let ans = state.top();
        Ok(ans.into_iter().collect())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut state = initial_state();
        input
            .lines()
            .skip(10)
            .map(|l| l.parse::<Move>().unwrap())
            .for_each(|m| state.process_enhanced(&m));
        let ans = state.top();
        Ok(ans.into_iter().collect())
    }
}

struct State<T> {
    s: Vec<Vec<T>>,
}

impl<T> State<T>
where
    T: Clone + std::fmt::Debug,
{
    fn process(&mut self, m: &Move) {
        if m.from == m.to {
            return;
        }
        for _ in 0..m.nr {
            let tmp = self.s[m.from - 1].pop().unwrap();
            self.s[m.to - 1].push(tmp);
        }
    }

    fn process_enhanced(&mut self, m: &Move) {
        if m.from == m.to {
            return;
        }
        let mut v = vec![];
        for _ in 0..m.nr {
            let tmp = self.s[m.from - 1].pop().unwrap();
            v.push(tmp);
        }
        v.reverse();
        self.s[m.to - 1].append(&mut v);
    }

    fn top(&self) -> Vec<T> {
        let mut ans = vec![];
        for v in self.s.clone() {
            ans.push(v.last().unwrap().clone());
        }
        ans
    }
}

#[derive(Debug)]
struct Move {
    nr: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" ").filter_map(|val| val.parse().ok());
        Ok(Move {
            nr: it.next().ok_or(format!("failed to parse moves"))?,
            from: it.next().ok_or(format!("failed to parse from"))?,
            to: it.next().ok_or(format!("failed to parse to"))?,
        })
    }
}

fn initial_state() -> State<char> {
    let v1 = vec!['B', 'Z', 'T'];
    let v2 = vec!['V', 'H', 'T', 'D', 'N'];
    let v3 = vec!['B', 'F', 'M', 'D'];
    let v4 = vec!['T', 'J', 'G', 'W', 'V', 'Q', 'L'];
    let v5 = vec!['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M'];
    let v6 = vec!['V', 'Z', 'Q', 'G', 'H', 'F', 'S'];
    let v7 = vec!['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W'];
    let v8 = vec!['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M'];
    let v9 = vec!['M', 'Q', 'L', 'F', 'D', 'S'];
    State {
        s: vec![v1, v2, v3, v4, v5, v6, v7, v8, v9],
    }
}
