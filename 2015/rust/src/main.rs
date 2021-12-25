mod utils;

mod day1;
mod day2;
mod day3;

fn main() -> std::io::Result<()> {
    day1::main()?;
    day2::main()?;
    day3::main()?;
    Ok(())
}
