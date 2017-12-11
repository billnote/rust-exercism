use std::collections::BTreeMap;

pub fn score(word: &str) -> u32 {
    let dic = build_dic();
    let uppercase_word = word.to_uppercase();
    let chars = uppercase_word.chars();

    chars.fold(0, |acc, c| acc + dic.get(&c).unwrap_or(&0))
}

fn build_dic() -> BTreeMap<char, u32> {
    [
        (1u32, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2u32, vec!['D', 'G']),
        (3u32, vec!['B', 'C', 'M', 'P']),
        (4u32, vec!['F', 'H', 'V', 'W', 'Y']),
        (5u32, vec!['K']),
        (8u32, vec!['J', 'X']),
        (10u32, vec!['Q', 'Z']),
    ].iter()
        .map(|&(v, ref chars)| {
            chars
                .iter()
                .scan(v, |state, c| Some((*c, *state)))
                .collect::<Vec<(char, u32)>>()
        })
        .collect::<Vec<Vec<(char, u32)>>>()
        .iter()
        .fold(vec![], |acc, v| {
            let mut total = acc.clone();
            let mut pairs = v.clone();
            total.append(&mut pairs);
            total
        })
        .iter()
        .map(|&(c, v)| (c, v))
        .collect()
}
