pub fn check(phrase: &str) -> bool {
    let mut sort_chars: Vec<char> = phrase
        .to_lowercase()
        .chars()
        .filter(|c| *c != ' ' && *c != '-')
        .collect::<Vec<char>>();
    sort_chars.sort();

    sort_chars.windows(2).all(
        |chars| chars.last() != chars.first(),
    )
}
