// Problem: Weekly Contest 429 Problem 1
// Translated from C++ to Rust

use std::collections::HashSet;
use std::io::{self, BufRead};

// Structure to replicate the functionality of the Solution class in C++
struct Solution;

impl Solution {
    // Method that implements the logic of minimumOperations
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        // Iterate the array from the end to the beginning
        for i in (0..nums.len()).rev() {
            // If the current number is already in the set
            if !seen.insert(nums[i]) {
                // Calculate and return the result following the C++ logic
                return (i / 3 + 1) as i32;
            }
        }
        // If no duplicate is found, return 0
        0
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse array size");

    // Read the elements of the array
    let mut nums = Vec::new();
    if let Some(line) = lines.next() {
        nums = line
            .expect("Failed to read array elements")
            .trim()
            .split_whitespace()
            .take(n) // Ensure only 'n' elements are parsed
            .map(|x| x.parse::<i32>().expect("Failed to parse array element"))
            .collect();
    } else {
        panic!("Expected input for array elements");
    }

    // Ensure the input size matches the declared size
    if nums.len() != n {
        panic!("Mismatch between declared size and actual number of elements");
    }

    // Create Solution instance and call the method
    let solution = Solution;
    let result = solution.minimum_operations(nums);

    // Output the result
    println!("{}", result);
}