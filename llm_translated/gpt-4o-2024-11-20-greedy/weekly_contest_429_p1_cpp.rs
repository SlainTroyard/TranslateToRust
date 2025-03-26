// Problem: Weekly Contest 429 Problem 1
use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for i in (0..nums.len()).rev() {
            // Try to insert nums[i] into the set. If it already exists, return the result.
            if !seen.insert(nums[i]) {
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    let mut input = String::new();

    // Read the size of the array
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Failed to parse input size");

    // Read the elements of the array
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Ensure the input size matches the provided array size
    assert_eq!(nums.len(), n, "Input size does not match the number of elements");

    // Call the function and output the result
    let result = Solution::minimum_operations(nums);
    println!("{}", result);
}