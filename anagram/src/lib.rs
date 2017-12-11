pub fn anagrams_for<'a>(word: &'a str, list: &[&'a str]) -> Vec<&'a str> {
    let target_word: String = order_word_chars(word);
    list.iter()
        .map(|s| (s, order_word_chars(s)))
        .filter_map(|(s, ord)| if ord == target_word &&
            s.to_lowercase() != word.to_lowercase()
        {
            Some(*s)
        } else {
            None
        })
        .collect::<Vec<&'a str>>()
}

fn order_word_chars(word: &str) -> String {
    let mut target = word.to_lowercase().chars().collect::<Vec<char>>();
    target.sort();
    target.iter().collect()
}
