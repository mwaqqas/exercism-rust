pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits_of_num(num);

    let mut sum = 0;
    let len: u32 = digits.len() as u32;

    for n in digits {
        sum = sum + n.pow(len);
    }

    sum == num
}

fn get_digits_of_num(num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n = n / 10;
    }

    digits.reverse();

    digits
}
