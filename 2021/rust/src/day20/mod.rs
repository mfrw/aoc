use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day20/Part1 Sol: {}", part1(&input.0, &input.1));
    println!("Day20/Part2 Sol: {}", part2(&input.0, &input.1));

    Ok(())
}

fn run(alg: &[char], img: &Vec<Vec<char>>, nr: usize) -> usize {
    let mut img = img.clone();
    pad(&mut img, Some('.'));
    for _ in 0..nr {
        enhance(alg, &mut img)
    }
    img.into_iter()
        .map(|row| row.into_iter().filter(|&v| v == '#').count())
        .sum()
}

fn part1(alg: &[char], img: &Vec<Vec<char>>) -> usize {
    run(alg, img, 2)
}

fn part2(alg: &[char], img: &Vec<Vec<char>>) -> usize {
    run(alg, img, 50)
}

fn pad(grid: &mut Vec<Vec<char>>, ch: Option<char>) {
    let c = ch.unwrap_or(grid[0][0]);
    grid.insert(0, vec![c; grid[0].len()]);
    grid.push(vec![c; grid[0].len()]);
    for row in grid.iter_mut() {
        row.insert(0, c);
        row.push(c);
    }
}

fn num(bs: Vec<char>) -> usize {
    bs.into_iter().fold(0, |a, b| a << 1 | (b == '#') as usize)
}

fn enhance(iea: &[char], grid: &mut Vec<Vec<char>>) {
    pad(grid, None);
    let mut grid2 = grid.clone();
    for r in 1..grid.len() - 1 {
        for c in 1..grid[r].len() - 1 {
            let idx = num(vec![
                grid[r - 1][c - 1],
                grid[r - 1][c],
                grid[r - 1][c + 1],
                grid[r][c - 1],
                grid[r][c],
                grid[r][c + 1],
                grid[r + 1][c - 1],
                grid[r + 1][c],
                grid[r + 1][c + 1],
            ]);
            grid2[r][c] = iea[idx];
        }
    }
    std::mem::swap(grid, &mut grid2);
    let ch = iea[num(vec![grid[0][0]; 9])];
    let last = grid.len() - 1;
    for (i, row) in grid.iter_mut().enumerate() {
        row[0] = ch;
        *row.last_mut().unwrap() = ch;
        if i == 0 || i == last {
            row.iter_mut().for_each(|v| *v = ch);
        }
    }
}

fn get_input() -> Result<(Vec<char>, Vec<Vec<char>>), std::io::Error> {
    let input = utils::get_input("input/day20")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<(Vec<char>, Vec<Vec<char>>), std::io::Error> {
    let (algstr, imgstr) = input.split_once("\n\n").unwrap();
    let algo = algstr.chars().collect::<Vec<_>>();
    let img = imgstr
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok((algo, img))
}
