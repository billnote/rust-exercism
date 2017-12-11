pub fn factors(num: u64) -> Vec<u64> {
    let mut factors_vec = Vec::new();
    let mut next_num = num;
    let mut next_factor = 2;

    // It is not necessary to determine whether the factor is prime
    // better preformance
    while next_num >= next_factor {
        let remainder = next_num % next_factor;
        if remainder == 0 {
            factors_vec.push(next_factor);
            next_num /= next_factor;
        } else {
            if next_factor == 2 {
                next_factor += 1;
            } else {
                next_factor += 2;
            }
        }
    }

    factors_vec
}
