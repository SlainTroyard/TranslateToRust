// Problem: Weekly Contest 423 Problem 2
use std::io;

// Function to return the minimum of two integers
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Function to return the maximum of three integers
fn max(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b };
    if d > c { d } else { c }
}

// Function to find the length of the longest increasing subarrays
fn max_increasing_subarrays(nums: &[i32]) -> i32 {
    let mut max_len = 0;
    let mut i = 1;
    let mut max1 = 1;
    
    while i < nums.len() {
        let mut max2 = 1;
        // Find the length of the current increasing subarray
        while i < nums.len() && nums[i] > nums[i - 1] {
            max2 += 1;
            i += 1;
        }
        let temp = min(max1, max2);
        max_len = max(max_len, temp, max2 / 2);
        max1 = max2;
        i += 1;
    }
    max_len
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Read the size of the array
    io::stdin().read_line(&mut input)?;
    let nums_size: usize = input.trim().parse()?;
    
    input.clear();
    // Read the array elements
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    
    // Call the function and print the result
    let result = max_increasing_subarrays(&nums);
    println!("{}", result);
    
    Ok(())
}