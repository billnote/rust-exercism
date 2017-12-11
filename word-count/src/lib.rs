use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    input
        .to_lowercase()
        .split_whitespace()
        .filter_map(|s| {
            let word = String::from_utf8(
                s.as_bytes()
                    .iter()
                    .filter_map(|c| match *c {
                        // '0'...'9' | 'a'...'z'
                        48u8...57u8 | 97u8...122u8 => Some(*c),
                        _ => None,
                    })
                    .collect::<Vec<u8>>(),
            ).unwrap()
                .trim()
                .to_string();

            if word.len() == 0 { None } else { Some(word) }
        })
        .fold(&mut HashMap::new(), |acc, word| {
            let count = *acc.get(&word).unwrap_or(&0);
            acc.insert(word, count + 1);
            acc
        })
        .clone()
}
