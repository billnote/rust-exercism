use std::error::Error;
use std::fmt;
use std::cmp::Eq;

#[derive(Debug)]
pub struct NthPrimeError {}

impl fmt::Display for NthPrimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "find prime error!")
    }
}

impl Error for NthPrimeError {
    fn description(&self) -> &str {
        "can't find nth prime!"
    }
}

impl PartialEq for NthPrimeError {
    fn eq(&self, other: &NthPrimeError) -> bool {
        self == other
    }
}

impl Eq for NthPrimeError {}

pub fn nth(n: u32) -> Result<u64, NthPrimeError> {
    // new solution for better performance
    if n == 0 {
        return Err(NthPrimeError {});
    }

    let mut primes: Vec<u64> = Vec::new();
    let mut result = 2;

    for _ in 0..n {
        match next_prime(&mut primes) {
            Ok(p) => {
                result = p;
            }
            Err(e) => {
                println!("error: {}", e);
                return Err(NthPrimeError {});
            }
        }
    }

    Ok(result)
}

fn is_prime(n: u64, ordered_primes: &Vec<u64>) -> bool {
    let limit: u64 = (n as f64).sqrt() as u64;

    ordered_primes
        .iter()
        .take_while(|&prime| prime <= &limit)
        .all(|&prime| n % prime != 0)
}

fn next_prime(ordered_primes: &mut Vec<u64>) -> Result<u64, String> {
    let mut current_num = 0;

    if ordered_primes.len() > 0 {
        let last_prime = ordered_primes.last().unwrap();
        if *last_prime > 2u64 {
            current_num += last_prime + 2;
        } else {
            current_num += last_prime + 1;
        }
    } else {
        current_num += 2;
    }

    for i in current_num.. {
        if is_prime(i, ordered_primes) {
            ordered_primes.push(i);
            return Ok(i);
        }
    }

    Err("can't get next prime".to_string())
}
