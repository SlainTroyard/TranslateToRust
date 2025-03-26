// Problem: Weekly Contest 429 Problem 1
use std::io::{self, BufRead};

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Array to count occurrences of each number (constraints say 1 <= nums[i] <= 100)
    
    // Iterate through the array from right to left
    for (i, &num) in nums.iter().enumerate().rev() {
        count[num as usize] += 1;
        if count[num as usize] > 1 {
            // If we found a duplicate, calculate the minimum operations
            // The formula (i + 3) / 3 gives the ceiling of i/3
            return ((i as i32) + 3) / 3;
        }
    }
    
    // If no duplicates found, return 0
    0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    let result = minimum_operations(&nums);
    println!("{}", result);
}