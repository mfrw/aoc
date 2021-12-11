pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_input(pth: &str) -> Result<String> {
    let input = std::fs::read_to_string(pth)?;
    Ok(input)
}
