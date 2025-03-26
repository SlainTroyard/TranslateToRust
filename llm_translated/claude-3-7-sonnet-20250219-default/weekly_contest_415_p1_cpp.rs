// Problem: Weekly Contest 415 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn get_sneaky_numbers(nums: &Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 2;
        let mut xor_all = n as i32 ^ (n as i32 + 1);
        
        for i in 0..nums.len() {
            xor_all ^= i as i32 ^ nums[i];
        }
        
        // Find the number of trailing zeros (equivalent to __builtin_ctz in C++)
        let shift = xor_all.trailing_zeros() as usize;

        let mut ans = vec![0, 0];
        for i in 0..nums.len() {
            if i < n {
                let idx = ((i as i32) >> shift & 1) as usize;
                ans[idx] ^= i as i32;
            }
            let idx = (nums[i] >> shift & 1) as usize;
            ans[idx] ^= nums[i];
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line as numSize
    let mut num_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .parse()
        .expect("Failed to parse numSize");
    
    // Adjust numSize to include the 2 additional elements
    num_size += 2;
    
    // Read the second line as nums array
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    let solution = Solution;
    let result = Solution::get_sneaky_numbers(&nums);
    
    // Print the result
    for i in 0..2 {
        print!("{} ", result[i]);
    }
}