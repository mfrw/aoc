use crate::utils;

pub fn main() -> std::io::Result<()> {
    let input = get_input()?;
    println!("Day25/Part1 Sol: {}", part1(input));

    Ok(())
}

fn part1(grid: Vec<Vec<char>>) -> usize {
    let mut grid = grid.clone();
    let n = grid.len();
    let m = grid[0].len();
    for step in 1.. {
        let mut moved = false;
        for (f, (x, y)) in [('>', (0, 1)), ('v', (1, 0))] {
            let mut next = grid.clone();
            for i in 0..n {
                for j in 0..m {
                    if grid[i][j] == f && grid[(i + x) % n][(j + y) % m] == '.' {
                        next[(i + x) % n][(j + y) % m] = f;
                        next[i][j] = '.';
                        moved = true;
                    }
                }
            }
            grid = next;
        }
        if !moved {
            return step;
        }
    }
    unreachable!()
}

fn get_input() -> std::io::Result<Vec<Vec<char>>> {
    let input = utils::get_input("input/day25")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(part1(input), 337);
    }
}
