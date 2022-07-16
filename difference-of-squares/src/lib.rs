pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    let num_list = gen_list_of_nums(n);
    for i in num_list {
        sum += i;
    }

    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    let num_list = gen_list_of_nums(n);

    for i in num_list {
        sum += i * i
    }

    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn gen_list_of_nums(max_num: u32) -> Vec<u32> {
    let mut list_of_nums: Vec<u32> = Vec::with_capacity(max_num as usize);

    let mut count = 1;
    while count <= max_num {
        list_of_nums.push(count);
        count += 1;
    }

    list_of_nums
}
