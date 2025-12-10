use crate::utils;

pub struct Solver;

impl utils::Solver<10> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let machines = parse_input(input);
        let mut total_presses = 0;

        for machine in machines.iter() {
            match machine.solve_min_presses() {
                Some(presses) => {
                    total_presses += presses;
                }
                None => unreachable!("Impossible config"),
            }
        }
        Ok(total_presses)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
}

impl Machine {
    /// Solves for the minimum number of button presses.
    /// Iterates through combination sizes (0, 1, 2...) to ensure the first solution found is the smallest.
    fn solve_min_presses(&self) -> Option<usize> {
        let n = self.buttons.len();

        for k in 0..=n {
            if self.check_combination(0, k, 0) {
                return Some(k);
            }
        }
        None
    }

    /// Recursive helper (DFS) to check combinations of size `k_needed`
    fn check_combination(&self, start_idx: usize, k_needed: usize, current_val: u64) -> bool {
        // Base Case: We have selected exactly k buttons
        if k_needed == 0 {
            return current_val == self.target;
        }

        // Recursive Step: Try adding next buttons
        for i in start_idx..self.buttons.len() {
            // Optimization: If we don't have enough buttons left to reach k_needed, stop
            if (self.buttons.len() - i) < k_needed {
                break;
            }

            let new_val = current_val ^ self.buttons[i];
            if self.check_combination(i + 1, k_needed - 1, new_val) {
                return true;
            }
        }
        false
    }
}

/// Parses the input string manually without external crates.
fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // 1. Parse Target Configuration: Look for [...]
        let start_bracket = match line.find('[') {
            Some(i) => i,
            None => continue, // Invalid line
        };
        let end_bracket = match line.find(']') {
            Some(i) => i,
            None => continue,
        };

        let config_str = &line[start_bracket + 1..end_bracket];
        let mut target: u64 = 0;
        for (i, c) in config_str.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }

        // 2. Parse Buttons: Look for all occurrences of (...)
        let mut buttons = Vec::new();
        // We scan the rest of the string after the target diagram
        let mut search_slice = &line[end_bracket + 1..];

        while let Some(open_paren) = search_slice.find('(') {
            if let Some(close_paren) = search_slice[open_paren..].find(')') {
                // The actual close index in 'search_slice' is open_paren + close_paren
                let close_idx = open_paren + close_paren;

                let content = &search_slice[open_paren + 1..close_idx];
                let mut btn_mask: u64 = 0;

                // Content looks like "0,2,3"
                if !content.trim().is_empty() {
                    for num_str in content.split(',') {
                        if let Ok(idx) = num_str.trim().parse::<u64>() {
                            btn_mask |= 1 << idx;
                        }
                    }
                }
                buttons.push(btn_mask);

                // Advance the slice
                search_slice = &search_slice[close_idx + 1..];
            } else {
                break;
            }
        }

        machines.push(Machine { target, buttons });
    }
    machines
}
