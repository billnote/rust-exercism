const CHAR_A: u8 = 97;
const CHAR_Z: u8 = 122;

pub fn is_pangram(sentence: &str) -> bool {
    let mut chars = sentence.to_lowercase().as_bytes().to_vec();
    chars.sort();

    let mut sentry = CHAR_A;

    match chars.binary_search(&CHAR_A) {
        Ok(idx) => {
            let sub_chars = chars.get(idx..chars.len()).unwrap();
            println!("{:?}", sub_chars);

            for c in sub_chars {
                if c > &sentry {
                    return false;
                } else if c == &CHAR_Z {
                    return true;
                } else if c == &sentry {
                    sentry += 1;
                }
            }

            false
        }
        Err(_) => false,
    }
}
