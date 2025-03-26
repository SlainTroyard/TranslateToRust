// Problem: Weekly Contest 434 Problem 1
use std::io::{self, BufRead};
use std::iter::Sum;

struct Solution;

impl Solution {
    /// Counts the partitions based on the provided algorithm.
    /// 
    /// # Arguments
    /// - `nums`: A vector of integers representing the input.
    /// 
    /// # Returns
    /// Returns the computed result as an integer, following the logic:
    /// - If the total sum of the elements is odd, return 0.
    /// - Otherwise, return the size of the vector minus 1.
    fn count_partitions(nums: Vec<i32>) -> usize {
        let sum: i32 = nums.iter().sum(); // Calculate the sum of the vector elements.
        if sum % 2 != 0 {
            0 // If the sum is odd, no partitions are possible.
        } else {
            nums.len() - 1 // If the sum is even, return the size minus 1.
        }
    }
}

fn main() {
    // Read input exactly like the original C++ code.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // First line contains the integer n.
    let n: usize = iterator
        .next()
        .expect("Expected input for vector size")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected a valid integer");

    // Second line contains n space-separated integers.
    let nums: Vec<i32> = iterator
        .next()
        .expect("Expected input for vector elements")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected a valid integer"))
        .collect();

    // Ensure input length matches the given `n`.
    assert_eq!(nums.len(), n, "Mismatch between declared and actual vector size");

    // Create an instance of the Solution struct and call the function.
    let sol = Solution;
    let result = sol.count_partitions(nums);

    // Output the result exactly like the original C++ code.
    println!("{}", result);
}