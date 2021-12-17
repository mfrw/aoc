use crate::utils;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!(
        "Day17/Part1 Sol: {}",
        part1(input.0 .0, input.0 .1, input.1 .0, input.1 .1)
    );
    Ok(())
}

fn part1(x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
    heights(x0, x1, y0, y1).max().unwrap()
}

fn max_height(x0: i32, x1: i32, y0: i32, y1: i32, mut vx: i32, mut vy: i32) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;
    while y >= y0 {
        if x0 <= x && x <= x1 && y0 <= y && y <= y1 {
            return Some(max_y);
        }
        x += vx;
        y += vy;
        max_y = std::cmp::max(max_y, y);
        vx = std::cmp::max(0, vx - 1);
        vy -= 1;
    }
    None
}

// Finally I am improving an moving beyond the novice noob into an informed beginer!
// Happy ME :)
fn heights(x0: i32, x1: i32, y0: i32, y1: i32) -> impl Iterator<Item = i32> {
    let mx = ((1.0 + 8.0 * x0 as f64).sqrt() / 2.0 - 0.5).ceil() as i32;
    (mx..=x1).flat_map(move |x| (y0..=-y0).filter_map(move |y| max_height(x0, x1, y0, y1, x, y)))
}

fn get_input() -> Result<((i32, i32), (i32, i32)), std::io::Error> {
    let input = utils::get_input("input/day17")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<((i32, i32), (i32, i32)), std::io::Error> {
    let (xstr, ystr) = input
        .trim_matches('\n')
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();
    let x = xstr.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let x = (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap());
    let y = ystr.strip_prefix("y=").unwrap().split_once("..").unwrap();
    let y = (y.0.parse::<i32>().unwrap(), y.1.parse::<i32>().unwrap());
    Ok((x, y))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let got = part1(217, 240, -126, -69);
        assert_eq!(7875, got);
    }
}
