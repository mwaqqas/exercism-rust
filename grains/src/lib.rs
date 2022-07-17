pub fn square(s: u32) -> u64 {
    let mut grains: Vec<u64> = Vec::new();

    match s {
        1..=64 => {
            for i in 0..s as usize {
                match i {
                    0 => grains.push(1),
                    1 => grains.push(2),
                    _ => {
                        let prev = grains[i - 1];
                        grains.push(prev * 2);
                    }
                }
            }
        }
        _ => panic!("Square must be between 1 and 64"),
    }

    grains[(s - 1) as usize]
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=64 {
        sum = sum + square(i);
    }

    sum
}
