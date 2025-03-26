/// Weekly Contest 423 Problem 1 in Rust
/// 
/// This program reads:
/// 1) An integer n (the size of the array),
/// 2) n integers (the elements of the array),
/// 3) An integer k (the length of the subarray),
/// and checks if there exist two disjoint strictly increasing subarrays
/// of length k such that the first ends before the second begins.
/// 
/// It then prints "true" or "false" depending on whether such subarrays exist.

use std::io::{self, Read};

/// Checks if there exist two disjoint strictly increasing subarrays
/// each of length k within 'nums'. Returns true if found, false otherwise.
fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    // A closure that checks if the subarray of length k starting at 'idx' is strictly increasing
    let mono = |idx: usize| -> bool {
        for i in idx..(idx + k - 1) {
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    };

    // Iterate through all possible starting indices to find two disjoint subarrays
    // The second subarray starts at 'idx + k'
    for idx in 0..=nums.len().saturating_sub(2 * k) {
        if mono(idx) && mono(idx + k) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all input from stdin at once
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input by whitespace into tokens
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Parse n (size of the array)
    let n: usize = tokens[0].parse()?;

    // Parse the n array elements
    let mut nums = Vec::with_capacity(n);
    let mut index = 1;
    for _ in 0..n {
        let val: i32 = tokens[index].parse()?;
        nums.push(val);
        index += 1;
    }

    // Parse k (the subarray length)
    let k: usize = tokens[index].parse()?;

    // Check for two strictly increasing subarrays of length k
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}