mod utils;

mod day1;
mod day2;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::main()?;
    day2::main()?;
    Ok(())
}
