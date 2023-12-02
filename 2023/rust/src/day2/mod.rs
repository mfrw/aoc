use crate::utils;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub struct Solver;

impl utils::Solver<2> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part2_int(input: &str) -> Option<usize> {
    todo!()
}

fn part1_int(input: &str) -> Option<usize> {
    let mut ans = 0;
    for line in input.lines() {
        if let Ok((_, game)) = parse_game_line(line) {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for rnd in game.rounds {
                r = r.max(rnd.red);
                g = g.max(rnd.green);
                b = b.max(rnd.blue);
            }
            if r > 12 || g > 13 || b > 14 {
                continue;
            }
            ans += game.id;
        }
    }
    Some(ans)
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<GameRound>,
}

#[derive(Debug)]
struct GameRound {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
enum Cube {
    R(usize),
    G(usize),
    B(usize),
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn sp(i: &str) -> IResult<&str, &str> {
    multispace0(i)
}

fn parse_cube(i: &str) -> IResult<&str, Cube> {
    let (i, (nr, _, clr)) = tuple((
        parse_number,
        sp,
        alt((tag("green"), tag("red"), tag("blue"))),
    ))(i)?;
    let c = match clr {
        "green" => Cube::G(nr),
        "red" => Cube::R(nr),
        "blue" => Cube::B(nr),
        _ => unimplemented!(),
    };
    Ok((i, c))
}

fn parse_round(i: &str) -> IResult<&str, GameRound> {
    let (i, v) = separated_list0(tag(", "), parse_cube)(i)?;
    let mut gr = GameRound {
        green: 0,
        red: 0,
        blue: 0,
    };
    for c in v {
        match c {
            Cube::R(x) => gr.red += x,
            Cube::G(x) => gr.green += x,
            Cube::B(x) => gr.blue += x,
        }
    }
    Ok((i, gr))
}

fn parse_game_line(i: &str) -> IResult<&str, Game> {
    let (i, (_, id, _)) = tuple((tag("Game "), parse_number, tag(": ")))(i)?;
    //println!("GameID: {id}");
    let (i, rounds) = separated_list0(preceded(sp, tag("; ")), parse_round)(i)?;
    //println!("{:?}", rounds);
    let game = Game { id, rounds };
    Ok((i, game))
}

#[test]
fn p1_test() {
    let i = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1_int(i), Some(8))
}
