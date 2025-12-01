use crate::utils;

pub struct Solver;

impl utils::Solver<13> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        Ok(part1_int(input).unwrap())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(part2_int(input).unwrap())
    }
}

fn part1_int(input: &str) -> Option<usize> {
    let landscapes = generate(input);
    Some(landscapes.iter().map(reflection_score).sum())
}

fn part2_int(input: &str) -> Option<usize> {
    todo!()
}

#[derive(Debug)]
pub struct Landscape {
    cols: Vec<u32>,
    rows: Vec<u32>,
}

pub fn generate(input: &str) -> Vec<Landscape> {
    input
        .split("\n\n")
        .map(|landscape| {
            let col_len = landscape.find('\n').unwrap();
            let mut cols = vec![0; col_len];
            let mut rows = vec![0; 1];
            let mut col_idx = 0;
            let mut row_idx = 0;
            for ch in landscape.chars() {
                if ch == '\n' {
                    rows.push(0);
                    row_idx += 1;
                    col_idx = 0;
                    continue;
                }
                let val = (ch == '#') as u32;
                unsafe {
                    let c = cols.get_unchecked_mut(col_idx);
                    *c <<= 1;
                    *c += val;
                    let r = rows.get_unchecked_mut(row_idx);
                    *r <<= 1;
                    *r += val;
                }

                col_idx += 1;
            }
            Landscape { rows, cols }
        })
        .collect()
}

fn try_reflect(images: &[u32]) -> usize {
    let mut reflected = false;
    let mut idx = 1;
    while !reflected && idx < images.len() {
        reflected = true;
        let mut lo = idx;
        let mut hi = idx - 1;
        while lo > 0 && hi < images.len() - 1 {
            lo -= 1;
            hi += 1;
            if images[lo] != images[hi] {
                reflected = false;
                break;
            }
        }
        if reflected {
            break;
        }
        idx += 1;
    }

    if reflected {
        idx
    } else {
        0
    }
}

fn reflection_score(landscape: &Landscape) -> usize {
    let vert = try_reflect(&landscape.cols);
    if vert > 0 {
        vert
    } else {
        let horiz = try_reflect(&landscape.rows);
        horiz * 100
    }
}
