#![feature(binary_heap_into_iter_sorted)]

mod utils;

mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::main()?;
    Ok(())
}
