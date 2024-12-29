pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() || digits.is_empty() {
        return vec![];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|windows| windows.iter().collect::<String>())
        .collect()
}
