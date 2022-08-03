pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut n = n;
            let mut steps: u64 = 0;

            while n != 1 {
                steps += 1;

                match n % 2 == 0 {
                    true => n /= 2,
                    false => n = n.checked_mul(3)?.checked_add(1)?,
                }
            }

            Some(steps)
        }
    }
}
