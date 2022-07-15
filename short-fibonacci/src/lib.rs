use std::vec;

const MAX_FIB_LEN: usize = 5;

pub fn create_empty() -> Vec<u8> {
    return Vec::new();
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    return vec![0; count];
}

pub fn fibonacci() -> Vec<u8> {
    let mut fib = create_buffer(MAX_FIB_LEN);

    for i in 0..MAX_FIB_LEN {
        match i {
            0 | 1 => fib[i] = 1,
            _ => fib[i] = fib[i - 1] + fib[i - 2],
        }
    }

    return fib;
}
