extern crate num_bigint as bigint;

use bigint::BigInt;

use std::cmp::PartialEq;
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Eq, Debug)]
pub struct Decimal {
    point_len: usize,
    digit: BigInt,
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let (_, a, b) = Decimal::to_same_point(self, other);
        a.eq(&b)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let (_, a, b) = Decimal::to_same_point(self, other);
        a.cmp(&b)
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (point, a, b) = Decimal::to_same_point(&self, &rhs);
        Decimal {
            point_len: point,
            digit: a + b,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (point, a, b) = Decimal::to_same_point(&self, &rhs);
        Decimal {
            point_len: point,
            digit: a - b,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (point, a, b) = Decimal::to_same_point(&self, &rhs);
        Decimal {
            point_len: point * 2,
            digit: a * b,
        }
    }
}

impl Decimal {
    pub fn try_from(num: &str) -> Option<Decimal> {
        let values: Vec<&str> = num.split_terminator('.').collect();
        match values.len() {
            1 => Decimal::try_from_with_point_len(num.as_bytes(), 0),
            2 => {
                Decimal::try_from_with_point_len(
                    (values[0].to_string() + values[1]).as_bytes(),
                    values[1].len(),
                )
            }
            _ => None,
        }
    }

    fn try_from_with_point_len(buf: &[u8], point_len: usize) -> Option<Decimal> {
        let digit = BigInt::parse_bytes(buf, 10);
        match digit {
            Some(d) => {
                Some(Decimal {
                    point_len: point_len,
                    digit: d,
                })
            }
            None => None,
        }
    }

    fn to_same_point(a: &Decimal, b: &Decimal) -> (usize, BigInt, BigInt) {
        let mut a_digit = a.digit.clone();
        let mut b_digit = b.digit.clone();
        let mut point_len = a.point_len;

        if a.point_len > b.point_len {
            for _ in 0..a.point_len - b.point_len {
                b_digit = b_digit * 10;
            }
        } else if a.point_len < b.point_len {
            for _ in 0..b.point_len - a.point_len {
                a_digit = a_digit * 10;
            }
            point_len = b.point_len;
        }
        (point_len, a_digit, b_digit)
    }
}
