# Advent of Code Solutions

This repository contains solutions to [Advent of Code](https://adventofcode.com/) challenges implemented in Rust.

## Overview

Advent of Code is an annual coding event with daily programming puzzles throughout December. This repository tracks solutions across multiple years, with each year containing implementations for various daily challenges.

## Repository Structure

The repository is organized by year, with each year containing Rust implementations:

```
aoc/
├── 2015/rust/    # 9 days solved
├── 2016/rust/    # 3 days solved
├── 2020/rust/    # 3 days solved
├── 2021/rust/    # 25 days solved ✓ (Complete)
├── 2022/rust/    # 25 days solved ✓ (Complete)
├── 2023/rust/    # 13 days solved
├── 2024/rust/    # 2 days solved
└── 2025/rust/    # 12 days solved
```

Each year's directory follows this structure:
- `Cargo.toml` - Rust project configuration
- `src/` - Source code directory
  - `main.rs` - Entry point that runs all solved days
  - `dayN/mod.rs` - Individual day solutions
  - `utils/mod.rs` - Shared utility functions and traits
- `input/` - Puzzle input files (one per day)

## Solution Architecture

All solutions follow a consistent pattern using a custom `Solver` trait:

```rust
pub trait Solver<const DAY: usize> {
    type Part1: Display;
    type Part2: Display;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>>;
    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>>;
    
    fn solve(&self) -> Result<(), Box<dyn std::error::Error>>;
}
```

Each day implements this trait, providing solutions for both Part 1 and Part 2 of the daily puzzle. The `solve()` method automatically:
- Reads the input file
- Times both parts
- Prints results with execution time

## Running Solutions

To run solutions for a specific year:

```bash
cd YYYY/rust/
cargo run --release
```

This will execute all implemented days for that year and display timing information.

## Dependencies

The solutions use minimal external dependencies, primarily:
- `itertools` - Iterator utilities
- `nom` - Parser combinators (for complex parsing)
- `rayon` - Parallel iterators (for performance)
- `scan_fmt` - String scanning utilities

## Progress

Completed Years:
- ✅ 2021 - All 25 days
- ✅ 2022 - All 25 days

In Progress:
- 2025 - 12/25 days
- 2023 - 13/25 days
- 2015 - 9/25 days
- 2016 - 3/25 days
- 2020 - 3/25 days
- 2024 - 2/25 days

## License

This is free and unencumbered software released into the public domain. See [LICENSE](LICENSE) file for details.

## Author

Muhammad Falak R Wani (falakreyaz@gmail.com)

## About Advent of Code

Advent of Code is created by [Eric Wastl](http://was.tl/). Visit [adventofcode.com](https://adventofcode.com/) to participate!
