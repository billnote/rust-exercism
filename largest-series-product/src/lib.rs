pub fn lsp(number_str: &str, n: usize) -> Result<u32, String> {

    if n == 0 {
        Ok(1)
    } else if number_str.chars().any(|c| !c.is_digit(10)) {
        Err(String::from("String with non digit"))
    } else {
        match number_str
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(1))
            .collect::<Vec<u32>>()
            .windows(n)
            .map(|series| series.iter().product())
            .max() {
            Some(l) => Ok(l),
            None => Err(String::from("Can't find largest series product!")),
        }
    }
}
