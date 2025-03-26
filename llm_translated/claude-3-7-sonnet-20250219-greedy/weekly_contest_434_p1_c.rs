// Problem: Weekly Contest 434 Problem 1
use std::io::{self, BufRead};

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        let left_sum: i32 = nums[0..=i].iter().sum();
        let right_sum: i32 = nums[i+1..].iter().sum();
        
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse the number of elements");
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse an integer"))
        .collect();
    
    // Calculate and print the result
    println!("{}", count_partitions(&nums));
    
    Ok(())
}