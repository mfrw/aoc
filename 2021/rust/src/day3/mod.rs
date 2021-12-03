use crate::utils;
use std::collections::HashMap;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let sol1 = part1(&input);
    println!("Day3/Part1 Sol: {}", sol1);
    Ok(())
}

fn part1(input: &[String]) -> usize {
    let mut mp: HashMap<usize, Vec<usize>> = HashMap::new();
    for data in input {
        for (idx, v) in data.chars().enumerate() {
            if v == '1' {
                mp.entry(idx)
                    .and_modify(|v| v[1] += 1)
                    .or_insert([0, 1].into());
            } else {
                mp.entry(idx)
                    .and_modify(|v| v[0] += 1)
                    .or_insert([1, 0].into());
            }
        }
    }

    let mut gama_vec: Vec<char> = vec!['0'; mp.len()];
    let mut alpha_vec: Vec<char> = vec!['1'; mp.len()];
    for (idx, v) in mp {
        if v[0] < v[1] {
            gama_vec[idx] = '1';
            alpha_vec[idx] = '0';
        }
    }
    let gama = usize::from_str_radix(&gama_vec.iter().collect::<String>(), 2).unwrap();
    let alpha = usize::from_str_radix(&alpha_vec.iter().collect::<String>(), 2).unwrap();
    gama * alpha
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    let input = utils::get_input("input/day3")?;
    let mut res: Vec<String> = vec![];
    for i in input.lines() {
        res.push(i.to_string());
    }
    Ok(res)
}
