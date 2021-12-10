use crate::utils;
use crate::utils::stack::Stack;

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day10/Part1 Sol: {}", part1(&input));
    println!("Day10/Part2 Sol: {}", part2(&input));
    Ok(())
}

fn part2(input: &[Vec<char>]) -> usize {
    let mut result = vec![];

    for line in input {
        let mut stk: Stack<char> = Stack::new();
        let mut broken = false;
        for ch in line.iter() {
            if matches!(ch, '(' | '[' | '{' | '<') {
                stk.push(*ch);
            } else {
                if stk.is_empty() {
                    broken = true;
                    break;
                } else {
                    match *ch {
                        ')' if stk.top() == Some(&'(') => stk.pop(),
                        ']' if stk.top() == Some(&'[') => stk.pop(),
                        '}' if stk.top() == Some(&'{') => stk.pop(),
                        '>' if stk.top() == Some(&'<') => stk.pop(),
                        _ => {
                            broken = true;
                            break;
                        }
                    };
                }
            }
        }
        if !broken {
            result.push(stk);
        }
    }
    let mut sorted = result
        .into_iter()
        .map(|stk| {
            stk.into_iter()
                .map(|ch| match ch {
                    '(' => 1usize,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, v| (acc * 5) + v)
        })
        .collect::<Vec<_>>();
    sorted.sort();
    if !sorted.is_empty() {
        return sorted[sorted.len() / 2];
    }
    unreachable!()
}

fn part1(input: &[Vec<char>]) -> usize {
    let mut result = vec![];
    for line in input {
        let mut stk: Stack<char> = Stack::new();
        for ch in line.iter() {
            if matches!(ch, '(' | '[' | '{' | '<') {
                stk.push(*ch);
            } else {
                if stk.is_empty() {
                    result.push(*ch);
                    break;
                } else {
                    match *ch {
                        ')' if stk.top() == Some(&'(') => stk.pop(),
                        ']' if stk.top() == Some(&'[') => stk.pop(),
                        '}' if stk.top() == Some(&'{') => stk.pop(),
                        '>' if stk.top() == Some(&'<') => stk.pop(),
                        _ => {
                            result.push(*ch);
                            break;
                        }
                    };
                }
            }
        }
    }
    result
        .into_iter()
        .map(|ch| match ch {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn get_input() -> Result<Vec<Vec<char>>, std::io::Error> {
    let input = utils::get_input("input/day10")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let v = input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_custom_test() {
        let raw_input = " [({(<(())[]>[[{[]{<()<>>\n [(()[<>])]({[<{<<[]>>(\n {([(<{}[<>[]}>{[]{[(<()>\n (((({<>}<{<{<>}{[]{[]{}\n [[<[([]))<([[{}[[()]]]\n [{[{({}]{}}([{[{{{}}([]\n {<[[]]>}<{[{[{[]{()[[[]\n [<(<(<(<{}))><([]([]()\n <{([([[(<>()){}]>(<<{{\n <{([{{}}[<[[[<>{}]]]>[]]\n";
        let input = parse_input(&raw_input).unwrap();
        assert_eq!(26397, part1(&input));
    }

    #[test]
    fn part1_test() {
        let input = get_input().unwrap();
        assert_eq!(392097, part1(&input));
    }

    #[test]
    fn part2_custom_test() {
        let raw_input = " [({(<(())[]>[[{[]{<()<>>\n [(()[<>])]({[<{<<[]>>(\n {([(<{}[<>[]}>{[]{[(<()>\n (((({<>}<{<{<>}{[]{[]{}\n [[<[([]))<([[{}[[()]]]\n [{[{({}]{}}([{[{{{}}([]\n {<[[]]>}<{[{[{[]{()[[[]\n [<(<(<(<{}))><([]([]()\n <{([([[(<>()){}]>(<<{{\n <{([{{}}[<[[[<>{}]]]>[]]\n";
        let input = parse_input(&raw_input).unwrap();
        assert_eq!(288957, part2(&input));
    }
    #[test]
    fn part2_test() {
        let input = get_input().unwrap();
        assert_eq!(4263222782, part2(&input));
    }
}
