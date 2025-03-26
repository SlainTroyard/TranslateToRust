// Problem: Weekly Contest 423 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: &Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        
        // Closure to check if a subarray starting at idx is strictly increasing
        let mono = |idx: usize| -> bool {
            for i in idx..(idx + k - 1) {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Check all possible positions for the two subarrays
        for idx in 0..=(nums.len() - 2 * k) {
            if mono(idx) && mono(idx + k) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read the length of the subarray
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a Solution object and call the function
    if Solution::has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}