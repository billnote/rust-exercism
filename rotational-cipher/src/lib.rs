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

fn cipher(s: char, rot: u32) -> char {
    match s.to_digit(36) {
        Some(x) => {
            if x < 10 {
                s
            } else {
                // idx = ((x - 10) + 26 + rot) % 26
                let idx = ((x + 16 + rot) % 26) as usize;
                let e = CIPHER_CHAR[idx];
                if s.is_uppercase() {
                    e.to_uppercase().next().unwrap()
                } else {
                    e
                }
            }
        }
        None => s,
    }
}

pub fn rotate(source: &str, rot: u32) -> String {
    source.chars().map(|c| cipher(c, rot)).collect::<String>()
}
