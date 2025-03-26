use std::io;
use std::io::Read;

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
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", count_partitions(&nums));
}