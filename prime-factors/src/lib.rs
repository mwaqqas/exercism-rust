pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    match (2..=n).find(|divisor| n % divisor == 0) {
        Some(x) => {
            result.push(x);
            result.append(&mut factors(n / x));
        }
        None => {}
    }

    result
}
