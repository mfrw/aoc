use crate::utils;

pub struct Day;

impl utils::Solver<8> for Day {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(i.lines().map(|l| l.len() - string_memory_size(l)).sum())
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(i.lines().map(|l| encode_string(l).len() - l.len()).sum())
    }
}

fn string_memory_size(s: &str) -> usize {
    let mut count = 0;

    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        let c = chars.next().unwrap();

        if c == '"' {
            continue;
        } else if c == '\\' {
            let next = chars.next().unwrap();
            match next {
                '\\' => {}
                '"' => {}
                'x' => {
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                }
                _ => panic!("invalid escape sequence"),
            }
        }

        count += 1;
    }

    count
}

fn encode_string(s: &str) -> String {
    let mut encoded = s.chars().fold('"'.to_string(), |mut encoded, c| {
        match c {
            '"' => encoded.push_str(r#"\""#),
            '\\' => encoded.push_str(r#"\\"#),
            c => encoded.push(c),
        }
        encoded
    });

    encoded.push('"');
    encoded
}
