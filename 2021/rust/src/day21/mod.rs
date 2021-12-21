pub fn main() -> Result<(), std::io::Error> {
    println!("Day21/Part1 Sol: {}", part1(10, 3));
    Ok(())
}

fn part1(pos1: usize, pos2: usize) -> usize {
    let (mut pos1, mut pos2) = (pos1, pos2);
    let (mut p1, mut p2, mut die, mut nrolls) = (0, 0, 1, 0);
    loop {
        for _ in 0..3 {
            if die > 100 {
                die = 1;
            }
            pos1 += die;
            die = (die + 1) % 100;
        }
        nrolls += 3;
        while pos1 > 10 {
            pos1 -= 10
        }
        p1 += pos1;
        if p1 >= 1000 {
            break;
        }

        for _ in 0..3 {
            if die > 100 {
                die = 1;
            }
            pos2 += die;
            die = (die + 1) % 100;
        }
        nrolls += 3;
        while pos2 > 10 {
            pos2 -= 10
        }
        p2 += pos2;
        if p2 >= 1000 {
            break;
        }
    }
    nrolls * if p1 >= 1000 { p2 } else { p1 }
}

