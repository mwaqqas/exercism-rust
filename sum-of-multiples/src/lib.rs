use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashMap<u32, bool> = HashMap::new();

    for num in 1..limit {
        for factor in factors {
            if *factor != 0 && num % factor == 0 {
                multiples.insert(num, true);
            }
        }
    }

    let mut sum: u32 = 0;

    for entry in multiples {
        sum += entry.0;
    }

    sum
}
