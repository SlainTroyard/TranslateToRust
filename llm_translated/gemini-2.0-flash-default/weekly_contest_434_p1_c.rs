use std::io;

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        let mut left_sum = 0;
        let mut right_sum = 0;

        for j in 0..=i {
            left_sum += nums[j];
        }

        for j in i + 1..nums.len() {
            right_sum += nums[j];
        }

        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("{}", count_partitions(&nums));
}