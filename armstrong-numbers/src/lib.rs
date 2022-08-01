pub fn is_armstrong_number(num: u32) -> bool {
    let sum: u32 = num
        .to_string()
        .chars()
        .flat_map(|ch| ch.to_digit(10))
        .map(|digit| digit.pow(num.to_string().len() as u32))
        .sum::<u32>();

    sum == num
}
