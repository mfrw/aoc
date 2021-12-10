pub fn get_input(pth: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(pth)
}

pub mod stack;
