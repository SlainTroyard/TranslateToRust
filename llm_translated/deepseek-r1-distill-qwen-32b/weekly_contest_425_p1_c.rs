// Problem: Weekly Contest 425 Problem 1
use std::io;

fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
    let mut min_sum: Option<i32> = None;

    for window_size in l..=r {
        let window_size = window_size as usize;
        if window_size == 0 || window_size > nums.len() {
            continue;
        }

        let mut sum: i32 = nums[0..window_size].iter().sum();
        if sum > 0 {
            if let Some(current_min) = min_sum {
                if sum < current_min {
                    min_sum = Some(sum);
                }
            } else {
                min_sum = Some(sum);
            }
        }

        for i in window_size..nums.len() {
            sum += nums[i] - nums[i - window_size];
            if sum > 0 {
                if let Some(current_min) = min_sum {
                    if sum < current_min {
                        min_sum = Some(sum);
                    }
                } else {
                    min_sum = Some(sum);
                }
            }
        }
    }

    min_sum.unwrap_or(-1)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read numsSize");
    let nums_size: usize = input.trim().parse().expect("Invalid numsSize");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read nums");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read l and r");
    let lr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    let l = lr[0];
    let r = lr[1];

    let result = minimum_sum_subarray(&nums, l, r);
    println!("{}", result);
}