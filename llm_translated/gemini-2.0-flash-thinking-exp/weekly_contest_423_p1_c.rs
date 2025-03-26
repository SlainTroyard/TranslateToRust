// Problem: Weekly Contest 423 Problem 1 in Rust

use std::io;
use std::str::SplitWhitespace;

// Function to check if there are increasing subarrays
fn has_increasing_subarrays(nums: &[i32], k: i32) -> bool {
    if k == 1 {
        return true; // Single element subarrays are always increasing
    }

    let k_usize = k as usize;
    let nums_size = nums.len();

    for _ in 0..2 { // Loop twice as in the original C code
        for i in 0..(nums_size - 2 * k_usize + 1) {
            let s = i + k_usize;
            let mut a = 0;
            // Check each pair within the subarray for increasing order
            for z in 0..(k_usize - 1) {
                if nums[i + z] < nums[i + z + 1] && nums[s + z] < nums[s + z + 1] {
                    a += 1;
                }
            }
            if a == k_usize - 1 {
                return true; // Found a valid increasing subarray
            }
        }
    }
    false // No valid increasing subarray found
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_str = input.trim();
    let nums_vec: Vec<i32> = nums_str
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number in nums"))
        .collect();
    let nums = &nums_vec;

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: i32 = input.trim().parse().expect("Invalid input for k");

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}