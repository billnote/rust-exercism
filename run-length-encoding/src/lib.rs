use std::str::Chars;

pub fn encode(source: &str) -> String {
    if source.len() <= 1 {
        return String::from(source);
    }

    let mut chars = source.chars();
    let mut encode_str: String = String::new();
    let mut current_c: Option<char> = chars.next();

    loop {
        let (next_c, count) = count_same_chars(current_c, &mut chars);
        if count > 1 {
            encode_str.push_str(&count.to_string());
        }
        encode_str.push(current_c.unwrap());

        if next_c == None {
            break;
        } else {
            current_c = next_c;
        }
    }


    encode_str
}

fn count_same_chars(current_c: Option<char>, chars: &mut Chars) -> (Option<char>, u32) {
    let mut next_c = chars.next();
    let mut count = 1;

    while next_c != None && current_c == next_c {
        count += 1;
        next_c = chars.next();
    }

    (next_c, count)
}

pub fn decode(source: &str) -> String {
    if source.len() <= 1 {
        return String::from(source);
    }

    let mut chars = source.chars();

    let mut decode_str = String::new();

    loop {
        let (current_c, num) = convert_row_digit_to_num(&mut chars);
        if current_c == None {
            break;
        }
        if num > 0 {
            for _ in 0..num {
                decode_str.push(current_c.unwrap());
            }
        } else {
            decode_str.push(current_c.unwrap());
        }
    }

    decode_str
}



fn convert_row_digit_to_num(chars: &mut Chars) -> (Option<char>, u32) {
    let mut c = chars.next();
    let mut num_str = String::new();

    while c != None && c.unwrap().is_digit(10) {
        num_str.push(c.unwrap());
        c = chars.next();
    }

    if num_str.len() > 0 {
        (c, u32::from_str_radix(&num_str, 10).unwrap())
    } else {
        (c, 0)
    }
}
