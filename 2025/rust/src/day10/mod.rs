use crate::utils;
use std::collections::{HashSet, VecDeque};

pub struct Solver;

impl utils::Solver<10> for Solver {
    type Part1 = usize;

    type Part2 = i64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let machines = input
            .lines()
            .map(parse_line)
            .filter(|m| !m.lights.is_empty())
            .collect::<Vec<_>>();
        let mut total_presses = 0;
        for (line_idx, machine) in machines.iter().enumerate() {
            // Part 1: BFS
            match solve_part1_bfs(
                &machine.lights,
                &machine.buttons_p1_matrix(machine.lights.len()),
            ) {
                Some(p) => total_presses += p,
                None => eprintln!("Line {}: No solution for Part 1", line_idx + 1),
            }
        }
        Ok(total_presses)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let machines = input
            .lines()
            .map(parse_line)
            .filter(|m| !m.lights.is_empty())
            .collect::<Vec<_>>();
        let mut total_presses = 0;
        for (line_idx, machine) in machines.into_iter().enumerate() {
            // Part 2: ILP
            let p2_matrix = machine.buttons_p2_matrix(machine.joltage.len());
            // Only solve if we have buttons for P2 (which we always should if parsed correctly)
            match solve_part2(machine.joltage, p2_matrix) {
                Some(p) => total_presses += p,
                None => eprintln!("Line {}: No solution for Part 2", line_idx + 1),
            }
        }
        Ok(total_presses)
    }
}

struct Matrix<T> {
    data: Box<[T]>,
    offset: usize,
}

impl<T> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            data: vec![T::default(); rows * cols].into_boxed_slice(),
            offset: cols,
        }
    }

    fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.offset + col]
    }

    fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.offset + col] = value;
    }

    fn swap_rows(&mut self, r1: usize, r2: usize) {
        if r1 == r2 {
            return;
        }
        let cols = self.offset;
        for c in 0..cols {
            self.data.swap(r1 * cols + c, r2 * cols + c);
        }
    }

    fn row_slice(&self, row: usize) -> &[T] {
        let start = row * self.offset;
        &self.data[start..start + self.offset]
    }
}

struct Machine {
    lights: Vec<u8>,
    button_indices: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

impl Machine {
    fn buttons_p1_matrix(&self, size: usize) -> Matrix<u8> {
        let num_buttons = self.button_indices.len();
        // For BFS, we iterate buttons. Store buttons as rows for easy slicing.
        let mut matrix = Matrix::new(num_buttons, size);
        for (btn_idx, indices) in self.button_indices.iter().enumerate() {
            for &light_idx in indices {
                if light_idx < size {
                    matrix.set(btn_idx, light_idx, 1);
                }
            }
        }
        matrix
    }

    fn buttons_p2_matrix(&self, size: usize) -> Matrix<i128> {
        let num_buttons = self.button_indices.len();
        // For Linear Algebra, buttons are columns.
        let mut matrix = Matrix::new(size, num_buttons);
        for (btn_idx, indices) in self.button_indices.iter().enumerate() {
            for &req_idx in indices {
                if req_idx < size {
                    matrix.set(req_idx, btn_idx, 1);
                }
            }
        }
        matrix
    }
}

fn parse_line(line: &str) -> Machine {
    // Input format: [lights] (button1) (button2) ... {joltage}

    // Parse Lights: [ ... ]
    let lights_end = line.find(']').unwrap_or(0);
    let lights = if lights_end > 1 {
        line[1..lights_end]
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect()
    } else {
        Vec::new()
    };

    // Parse Buttons: ( ... )
    let mut button_indices = Vec::new();
    let remaining = if lights_end < line.len() {
        &line[lights_end + 1..]
    } else {
        ""
    };

    // Find stop for buttons: start of joltage '{'
    let buttons_end = remaining.find('{').unwrap_or(remaining.len());
    let buttons_part = &remaining[..buttons_end];

    let mut offset = 0;
    while let Some(s) = buttons_part[offset..].find('(') {
        let abs_s = offset + s;
        if let Some(e) = buttons_part[abs_s..].find(')') {
            let content = &buttons_part[abs_s + 1..abs_s + e];
            let indices: Vec<usize> = if content.trim().is_empty() {
                Vec::new()
            } else {
                content
                    .split(',')
                    .map(|n| n.trim().parse().unwrap_or(0))
                    .collect()
            };
            button_indices.push(indices);
            offset = abs_s + e + 1;
        } else {
            break;
        }
    }

    // Parse Joltage: { ... }
    let joltage = if let (Some(s), Some(e)) = (remaining.find('{'), remaining.find('}')) {
        let content = &remaining[s + 1..e];
        if content.trim().is_empty() {
            Vec::new()
        } else {
            content
                .split(',')
                .map(|n| n.trim().parse().unwrap_or(0))
                .collect()
        }
    } else {
        Vec::new()
    };

    Machine {
        lights,
        button_indices,
        joltage,
    }
}

// Part 1: BFS Solver
fn solve_part1_bfs(target: &[u8], buttons: &Matrix<u8>) -> Option<usize> {
    let num_lights = target.len();
    let start_state = vec![0u8; num_lights];

    if start_state == target {
        return Some(0);
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_state.clone(), 0));

    let mut visited = HashSet::new();
    visited.insert(start_state);

    while let Some((state, depth)) = queue.pop_front() {
        if state == target {
            return Some(depth);
        }

        // buttons are rows in P1 matrix
        let num_buttons = buttons.data.len() / buttons.offset;
        for b_idx in 0..num_buttons {
            let button = buttons.row_slice(b_idx);
            let mut next_state = state.clone();
            // Apply button (XOR)
            for i in 0..num_lights {
                next_state[i] ^= button[i];
            }

            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, depth + 1));
            }
        }
    }
    None
}

// Part 2: ILP Solver (Gaussian + Search)
fn solve_part2(target: Vec<i64>, buttons: Matrix<i128>) -> Option<i64> {
    let num_requirements = target.len();
    let num_buttons = buttons.offset; // buttons are columns in P2 matrix

    // Matrix in i128 for precision. [A | b]
    // We construct augmented matrix from input A
    let mut matrix = Matrix::new(num_requirements, num_buttons + 1);

    for r in 0..num_requirements {
        for c in 0..num_buttons {
            matrix.set(r, c, *buttons.get(r, c));
        }
        matrix.set(r, num_buttons, target[r] as i128);
    }

    // Fraction-free Gaussian Elimination (Forward)
    let mut pivot_row = 0;
    let mut pivot_cols = Vec::with_capacity(num_requirements);
    let mut is_pivot_col = vec![false; num_buttons];

    for (c, is_pivot) in is_pivot_col.iter_mut().enumerate() {
        if pivot_row >= num_requirements {
            break;
        }

        // Find pivot
        let mut row = pivot_row;
        while row < num_requirements && *matrix.get(row, c) == 0 {
            row += 1;
        }

        if row < num_requirements {
            matrix.swap_rows(pivot_row, row);
            let pivot_val = *matrix.get(pivot_row, c);

            for r in pivot_row + 1..num_requirements {
                let val_r_c = *matrix.get(r, c);
                if val_r_c != 0 {
                    let factor = val_r_c;
                    for k in c..=num_buttons {
                        let new_val =
                            *matrix.get(r, k) * pivot_val - *matrix.get(pivot_row, k) * factor;
                        matrix.set(r, k, new_val);
                    }
                }
            }

            pivot_cols.push(c);
            *is_pivot = true;
            pivot_row += 1;
        }
    }

    // Check consistency
    for r in pivot_row..num_requirements {
        if *matrix.get(r, num_buttons) != 0 {
            // Check if LHS is all zero
            let mut all_zero = true;
            for c in 0..num_buttons {
                if *matrix.get(r, c) != 0 {
                    all_zero = false;
                    break;
                }
            }
            if all_zero {
                return None;
            }
        }
    }

    // Identify free variables
    let free_vars: Vec<usize> = (0..num_buttons).filter(|&c| !is_pivot_col[c]).collect();

    let mut min_total: Option<i64> = None;
    let mut current_free_vals = vec![0i64; free_vars.len()];

    // Heuristics for search
    fn search_int(
        idx: usize,
        free_vars: &Vec<usize>,
        free_vals: &mut Vec<i64>,
        matrix: &Matrix<i128>,
        pivot_cols: &[usize],
        num_buttons: usize,
        min_total: &mut Option<i64>,
    ) {
        if idx == free_vars.len() {
            // Check validity and calculate total
            let mut valid = true;
            let mut x = vec![0i128; num_buttons];

            for (i, &fv) in free_vars.iter().enumerate() {
                x[fv] = free_vals[i] as i128;
            }

            // Back substitution for pivots
            // Iterate rows from bottom up in the pivot set.
            // pivot_cols[r] gives the pivot column for row r.
            let num_pivots = pivot_cols.len();

            for r in (0..num_pivots).rev() {
                let pc = pivot_cols[r];
                let pivot_val = *matrix.get(r, pc);

                let mut rhs = *matrix.get(r, num_buttons);
                for k in pc + 1..num_buttons {
                    rhs -= *matrix.get(r, k) * x[k];
                }

                if rhs % pivot_val != 0 {
                    valid = false;
                    break;
                }

                let val = rhs / pivot_val;
                if val < 0 {
                    valid = false;
                    break;
                }
                x[pc] = val;
            }

            if valid {
                let sum: i128 = x.iter().sum();
                if sum < i64::MAX as i128 {
                    let sum_i64 = sum as i64;
                    if min_total.is_none_or(|m| sum_i64 < m) {
                        *min_total = Some(sum_i64);
                    }
                }
            }
            return;
        }

        // Search range logic
        let limit = if free_vars.len() > 1 { 200 } else { 20000 };
        for v in 0..=limit {
            free_vals[idx] = v;
            search_int(
                idx + 1,
                free_vars,
                free_vals,
                matrix,
                pivot_cols,
                num_buttons,
                min_total,
            );
        }
    }

    search_int(
        0,
        &free_vars,
        &mut current_free_vals,
        &matrix,
        &pivot_cols,
        num_buttons,
        &mut min_total,
    );

    min_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_1() {
        let target = vec![0, 1, 1, 0];
        // buttons as Matrix (rows = buttons, cols = lights)
        let mut buttons = Matrix::new(6, 4);
        /*
        vec![0, 0, 0, 1],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 1],
        vec![1, 0, 1, 0],
        vec![1, 1, 0, 0],
        */
        let data = Box::new([
            0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0,
        ]);
        buttons.data = data;

        assert_eq!(solve_part1_bfs(&target, &buttons), Some(2));
    }

    #[test]
    fn test_part2_example_1() {
        let target = vec![3, 5, 4, 7];
        // buttons as Matrix (rows = reqs=4, cols = buttons=6)
        let mut buttons = Matrix::new(4, 6);
        /*
        Buttons (cols):
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
        */
        // Rows:
        /*
        R0: 0 0 0 0 1 1
        R1: 0 1 0 0 0 1
        R2: 0 0 1 1 1 0
        R3: 1 1 0 1 0 0
        */
        let data = vec![
            0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0,
        ];
        // Convert to i128
        buttons.data = data.into_iter().map(|x| x as i128).collect();

        assert_eq!(solve_part2(target, buttons), Some(10));
    }
}
