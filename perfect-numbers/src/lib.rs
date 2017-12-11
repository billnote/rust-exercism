#[derive(PartialEq)]
#[derive(Debug)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(num: u64) -> Result<Classification, &'static str> {
    match num {
        x if x <= 0 => Err("Number must be positive"),
        _ => {
            let limit: u64 = (num as f64).sqrt() as u64 + 1;
            let sum = (2..limit)
                .filter_map(|x| if num % x == 0 { Some(x) } else { None })
                .fold(1, |acc, x| {
                    let mut total = acc;
                    total += x;
                    let q = num / x;
                    if x != q {
                        total += q;
                    }
                    total
                });

            match sum {
                x if x == 1 => Ok(Classification::Deficient),
                x if x > num => Ok(Classification::Abundant),
                x if x < num => Ok(Classification::Deficient),
                _ => Ok(Classification::Perfect),
            }
        }
    }
}
