// Problem: Weekly Contest 433 Problem 1
use std::cmp::max;
use std::io::{self, BufRead};

/// Calculate the sum of values in specific subarrays based on the problem's rules
fn subarray_sum(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut ans = 0;
    let mut sums = vec![0; nums_size + 1];
    
    for i in 0..nums_size {
        sums[i + 1] = nums[i] + sums[i];
        // The key algorithm logic: take the sum of elements from max(0, i-nums[i]) to i
        ans += sums[i + 1] - sums[max(0, i as i32 - nums[i]) as usize];
    }
    
    return ans;
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the length of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", subarray_sum(&nums));
}