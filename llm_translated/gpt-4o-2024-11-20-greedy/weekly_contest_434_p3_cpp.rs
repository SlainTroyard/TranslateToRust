use std::cmp::max;
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        // Initialize variables for tracking frequencies and max values
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;

        // Iterate through the numbers in the input vector
        for &x in &nums {
            // Update f2 based on the maximum of previous values and whether x == k
            f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };

            // Update frequency table f1 and calculate maximum f1 value
            f1[x as usize] = max(f1[x as usize], f0) + 1;

            // Update f0 based on whether x == k
            f0 += if x == k { 1 } else { 0 };

            // Track the maximum value in f1
            max_f1 = max(max_f1, f1[x as usize]);
        }

        // Return the maximum between max_f1 and f2
        max(max_f1, f2)
    }
}

fn main() {
    // Input handling
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the first input line into integers n and k
    let parts: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = parts[0];
    let k = parts[1];

    // Read the second line for the array of numbers
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n as usize);

    // Call the function and output the result
    let solution = Solution;
    let result = solution.max_frequency(nums, k);

    // Print the result
    println!("{}", result);
}