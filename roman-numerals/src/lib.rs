use std::convert::From;
use std::fmt;

pub struct Roman {
    roman_num: String,
}

impl From<u32> for Roman {
    fn from(n: u32) -> Self {
        if n > 3000 || n < 1 {
            Roman { roman_num: String::new() }
        } else {
            let mut roman_num = String::new();
            let mut r = n;

            [
                // pre mid currnet: number
                (' ', ' ', 'M', 1000),
                ('M', 'D', 'C', 100),
                ('C', 'L', 'X', 10),
                ('X', 'V', 'I', 1),
            ].iter()
                .fold(&mut roman_num, |acc, &(pre, mid, current, num)| {
                    let q = r / num;
                    r = r % num;

                    match q {
                        9 => {
                            acc.push(current);
                            acc.push(pre);
                        }
                        x if x >= 5 => {
                            acc.push(mid);
                            for _ in 0..q - 5 {
                                acc.push(current);
                            }
                        }
                        4 => {
                            acc.push(current);
                            acc.push(mid);
                        }
                        _ => {
                            for _ in 0..q {
                                acc.push(current);
                            }
                        }
                    }

                    acc
                });

            Roman { roman_num: roman_num }
        }
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.roman_num)
    }
}
