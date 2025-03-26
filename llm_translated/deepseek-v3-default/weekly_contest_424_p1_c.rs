use std::io::{self, Write};

fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;

    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please enter a valid number");
        nums.push(num);
    }

    let result = count_valid_selections(&nums);
    println!("{}", result);
}