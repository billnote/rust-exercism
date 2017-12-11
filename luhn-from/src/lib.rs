extern crate luhn;

use std::convert::From;
use std::fmt::Display;

pub struct Luhn<T>
where
    T: Display,
{
    number: T,
}

impl<T: Display> Luhn<T> {
    pub fn is_valid(&self) -> bool {
        luhn::is_valid(&self.number.to_string())
    }
}

impl<T: Display> From<T> for Luhn<T> {
    fn from(f: T) -> Self {
        Luhn { number: f }
    }
}
