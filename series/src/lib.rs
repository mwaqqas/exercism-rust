pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|slice| slice.iter().collect::<String>())
        .collect::<Vec<String>>()
}
