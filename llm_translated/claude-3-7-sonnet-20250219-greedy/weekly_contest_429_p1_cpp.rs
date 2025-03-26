use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for i in (0..nums.len()).rev() {
            // If insertion fails (element already exists), return the result
            if !seen.insert(nums[i]) {
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the elements of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), n, "Number of elements doesn't match the specified size");

    // Call the function and output the result
    let solution = Solution;
    let result = Solution::minimum_operations(&nums);
    println!("{}", result);
}