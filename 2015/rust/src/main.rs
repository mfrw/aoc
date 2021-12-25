mod utils;

mod day1;
mod day2;

fn main() -> std::io::Result<()> {
    day1::main()?;
    day2::main()?;
    Ok(())
}
