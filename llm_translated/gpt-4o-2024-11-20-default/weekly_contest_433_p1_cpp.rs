// Problem: Weekly Contest 433 Problem 1
use std::io;
use std::cmp;

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // Compute the prefix sum array
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0;
        for i in 0..n {
            // Safely calculate the subarray sum using the prefix sums
            ans += s[i + 1] - s[cmp::max(i as i32 - nums[i], 0) as usize];
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Expected a positive integer");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected an integer"))
        .collect();

    assert!(nums.len() == n, "Input length mismatch");

    // Solve the problem and print the result
    let solution = Solution;
    let result = solution.subarray_sum(nums);
    println!("{}", result);
}