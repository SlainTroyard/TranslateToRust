// Problem: Weekly Contest 434 Problem 1
use std::io;

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        // Calculate left sum (elements from 0 to i)
        let left_sum: i32 = nums[0..=i].iter().sum();
        
        // Calculate right sum (elements from i+1 to end)
        let right_sum: i32 = nums[i+1..].iter().sum();
        
        // Check if the difference is even
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    // Read the number of elements in the array
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).expect("Failed to read input");
    let n: usize = n_input.trim().parse().expect("Invalid number");
    
    // Read the array elements
    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).expect("Failed to read input");
    let nums: Vec<i32> = nums_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    // Calculate and print the result
    println!("{}", count_partitions(&nums));
}