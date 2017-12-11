const CIPHER_CHAR: [char; 26] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
];

pub fn encode(source: &str) -> String {
    source
        .chars()
        .filter_map(|c| cipher(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|s| s.to_vec())
        .collect::<Vec<Vec<char>>>()
        .join(&' ')
        .into_iter()
        .collect::<String>()
}

pub fn decode(source: &str) -> String {
    source.chars().filter_map(|c| cipher(c)).collect::<String>()
}

fn cipher(s: char) -> Option<char> {
    match s.to_digit(36) {
        Some(x) => {
            if x < 10 {
                Some(s)
            } else {
                let idx = (35 - x) as usize;
                Some(CIPHER_CHAR[idx])
            }
        }
        None => None,
    }
}
