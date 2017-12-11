pub fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = (2..n + 1).collect::<Vec<u32>>();

    let mut idx = 0;
    let mut current_prime = *primes.get(idx).unwrap_or(&0);
    let mut last_num = *primes.last().unwrap_or(&0);

    while current_prime > 0 && current_prime.pow(2) <= last_num {
        primes.retain(|&x| x % current_prime != 0 || x == current_prime);

        idx += 1;
        current_prime = *primes.get(idx).unwrap_or(&0);
        last_num = *primes.last().unwrap_or(&0);
    }

    primes
}
