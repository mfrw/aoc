use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit0,
    combinator::{all_consuming, value},
    sequence::tuple,
    Finish, IResult,
};

use crate::utils;
pub struct Day;

impl utils::Solver<6> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut grid = vec![vec![false; 1000]; 1000];
        parse_input(i).for_each(|c| {
            for x in c.start.0..=c.end.0 {
                for y in c.start.1..=c.end.1 {
                    match c.op {
                        Op::Toggle => grid[x][y] = !grid[x][y],
                        Op::TurnOn => grid[x][y] = true,
                        Op::TurnOff => grid[x][y] = false,
                    }
                }
            }
        });
        let ans: usize = grid
            .into_iter()
            .map(|r| r.into_iter().filter(|x| *x).count())
            .sum();
        Ok(ans)
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut grid = vec![vec![0_usize; 1000]; 1000];
        parse_input(i).for_each(|c| {
            for x in c.start.0..=c.end.0 {
                for y in c.start.1..=c.end.1 {
                    match c.op {
                        Op::Toggle => grid[x][y] += 2,
                        Op::TurnOn => grid[x][y] += 1,
                        Op::TurnOff => grid[x][y] = grid[x][y].saturating_sub(1),
                    }
                }
            }
        });
        let ans: usize = grid.into_iter().map(|r| r.into_iter().sum::<usize>()).sum();
        Ok(ans)
    }
}

fn parse_input(i: &str) -> impl Iterator<Item = Cmd> + '_ {
    i.lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
}

#[derive(Debug)]
struct Cmd {
    op: Op,
    start: Point,
    end: Point,
}

#[derive(Clone, Debug)]
enum Op {
    Toggle,
    TurnOff,
    TurnOn,
}

#[derive(Debug)]
struct Point(usize, usize);

fn parse_point(i: &str) -> IResult<&str, Point> {
    let (i, o) = tuple((digit0, tag(","), digit0))(i)?;
    let p0 = o.0.parse().unwrap();
    let p1 = o.2.parse().unwrap();
    Ok((i, Point(p0, p1)))
}

fn parse_points(i: &str) -> IResult<&str, (Point, Point)> {
    let (i, o) = tuple((parse_point, tag(" through "), parse_point))(i)?;
    Ok((i, (o.0, o.2)))
}

fn parse_on_off(i: &str) -> IResult<&str, Cmd> {
    let (ipt, (_, op, (start, end))) = tuple((
        tag("turn "),
        alt((
            value(Op::TurnOff, tag("off ")),
            value(Op::TurnOn, tag("on ")),
        )),
        parse_points,
    ))(i)?;
    let c = Cmd { op, start, end };
    Ok((ipt, c))
}

fn parse_toggle(i: &str) -> IResult<&str, Cmd> {
    let (ipt, (_, (start, end))) = tuple((tag("toggle "), parse_points))(i)?;
    let c = Cmd {
        op: Op::Toggle,
        start,
        end,
    };
    Ok((ipt, c))
}

fn parse_line(i: &str) -> IResult<&str, Cmd> {
    alt((parse_toggle, parse_on_off))(i)
}
