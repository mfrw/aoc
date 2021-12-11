mod utils;
use utils::Result;

mod day1;
mod day2;

fn main() -> Result<()> {
    day1::main()?;
    day2::main()?;
    Ok(())
}
