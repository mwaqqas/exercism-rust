pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| factors.iter().any(|fac| *fac != 0 && num % fac == 0))
        .sum()
}
