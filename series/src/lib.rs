pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        (0..digits.len() + 1)
            .map(|_| String::from(""))
            .collect::<Vec<String>>()
    } else {
        digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>()
    }

}
