use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for i in (0..nums.len()).rev() {
            // If the number is already in the set (insertion fails), return the result
            if !seen.insert(nums[i]) {
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    // Read the size of the array
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: usize = n.trim().parse().expect("Invalid input");

    // Read the array elements
    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).expect("Failed to read input");
    let nums: Vec<i32> = nums_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Call the solution and print the result
    let solution = Solution;
    let result = Solution::minimum_operations(&nums);
    println!("{}", result);
}