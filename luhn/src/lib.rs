pub fn is_valid(number_str: &str) -> bool {
    let (number_chars, other_chars): (Vec<char>, Vec<char>) =
        number_str.chars().partition(|c| *c >= '0' && *c <= '9');

    number_chars.len() > 1 && other_chars.iter().all(|c| *c == ' ') &&
        number_chars
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, c)| if idx % 2 != 0 {
                let mut number = c.to_digit(10).unwrap_or(0) * 2;
                if number > 9 {
                    number = number / 10 + number % 10;
                }
                number
            } else {
                c.to_digit(10).unwrap_or(0)
            })
            .fold(0, |acc, num| acc + num) % 10 == 0
}
