pub fn get_input(p: &str) -> std::io::Result<String> {
    std::fs::read_to_string(p)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "File reading error"))
}
