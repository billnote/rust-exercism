pub fn abbreviate(s: &str) -> String {
    s.replace("-", " ")
        .split_whitespace()
        .map(|s| if s == s.to_uppercase() {
            let mut result = String::from(s);
            result.truncate(1);
            result
        } else {
            s.chars()
                .enumerate()
                .filter_map(|(idx, c)| if idx == 0 || c.is_uppercase() {
                    c.to_uppercase().next()
                } else {
                    None
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("")
}
