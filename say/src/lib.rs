//
// See Rust Language Specific Instructions
// below normal exercise description.
//
use std::collections::BTreeMap;

const TWENTY: u64 = 20;

const HUNDRED: u64 = 100;
const THOUSAND: u64 = 1_000;
const MILLION: u64 = 1_000_000;
const BILLION: u64 = 1_000_000_000;
const TRILLION: u64 = 1_000_000_000_000;
const QUADRILLION: u64 = 1_000_000_000_000_000;
const QUINTILLION: u64 = 1_000_000_000_000_000_000;

pub fn encode(num: u64) -> String {
    let number_dic = build_number_dic();

    encode_iter(num, &number_dic)
}

fn encode_iter(num: u64, number_dic: &BTreeMap<u64, String>) -> String {
    match num {
        x if x <= TWENTY => format!("{}", *number_dic.get(&x).unwrap()),
        x if x < HUNDRED => {
            let (quotient, remainder) = ((x / 10) * 10, x % 10);
            format!(
                "{}-{}",
                *number_dic.get(&quotient).unwrap(),
                *number_dic.get(&remainder).unwrap()
            )
        }
        x @ _ => {
            let (divisor, num_code) = match x {
                x if x < THOUSAND => (HUNDRED, "hundred"),
                x if x < MILLION => (THOUSAND, "thousand"),
                x if x < BILLION => (MILLION, "million"),
                x if x < TRILLION => (BILLION, "billion"),
                x if x < QUADRILLION => (TRILLION, "trillion"),
                x if x < QUINTILLION => (QUADRILLION, "quadrillion"),
                _ => (QUINTILLION, "quintillion"),
            };

            let (quotient, remainder) = (x / divisor, x % divisor);
            format!("{} {}{}", encode_iter(quotient, number_dic), num_code, {
                if remainder == 0 {
                    String::new()
                } else {
                    " ".to_string() + &encode_iter(remainder, number_dic)
                }
            })
        }
    }
}

fn build_number_dic() -> BTreeMap<u64, String> {
    let mut number_dic: BTreeMap<u64, String> = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ].iter()
        .scan(0, |state, &x| {
            let result = Some((*state, String::from(x)));
            *state = *state + 1;
            result
        })
        .collect();

    let mut number_dic2: BTreeMap<u64, String> = [
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety",
    ].iter()
        .scan(10, |state, &x| {
            *state = *state + 10;
            Some((*state, String::from(x)))
        })
        .collect();

    number_dic.append(&mut number_dic2);

    number_dic
}
