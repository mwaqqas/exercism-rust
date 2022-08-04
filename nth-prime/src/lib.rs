pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut prime_candidate: u32 = *primes.last().unwrap() + 1;

    while primes.len() <= n as usize {
        if is_prime(prime_candidate) {
            primes.push(prime_candidate);
        }
        prime_candidate += 1;
    }

    *primes.get((n) as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    let mut limit: u32 = n;
    if n > 10 {
        limit = n / 2;
    }

    for divisor in 2..limit {
        if n % divisor == 0 {
            return false;
        }
    }

    true
}
