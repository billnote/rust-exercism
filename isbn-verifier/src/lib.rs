pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .enumerate()
        .fold(0, |acc, pair| if pair.0 < 9 {
            acc + (10 - pair.0 as u32) * (pair.1.to_digit(10).unwrap_or(0))
        } else {
            if pair.1 == 'X' {
                acc + 10
            } else {
                acc + (pair.1.to_digit(10).unwrap_or(0))
            }
        }) % 11 == 0
}
