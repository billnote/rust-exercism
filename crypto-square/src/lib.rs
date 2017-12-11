pub fn encrypt(sentence: &str) -> String {
    let new_sentence = sentence
        .to_lowercase()
        .chars()
        .filter(|c| match *c {
            'a'...'z' => true,
            _ => false,
        })
        .collect::<String>();

    let c = (new_sentence.len() as f32).sqrt().ceil() as u32;
    let mut v: Vec<Vec<char>> = vec![vec![]; c as usize];

    new_sentence
        .chars()
        .enumerate()
        .fold(v.as_mut_slice(), |acc, pair| {
            let idx = (pair.0 as u32) % c;
            {
                let values = acc.get_mut(idx as usize).unwrap();
                values.push(pair.1);
            }
            acc
        })
        .iter()
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
