mod utils;

mod day1;
mod day2;
mod day3;
mod day4;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1::main()?;
    day2::main()?;
    day3::main()?;
    day4::main()?;
    Ok(())
}
