// Problem: Weekly Contest 424 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        // Compute prefix sums from the left
        for i in 1..n {
            left[i] = left[i - 1] + nums[i - 1];
        }

        // Compute prefix sums from the right
        for i in 1..n {
            right[n - i - 1] = right[n - i] + nums[n - i];
        }

        // Iterate through each element to count valid selections
        for i in 0..n {
            if nums[i] != 0 {
                continue;
            }
            if left[i] == right[i] {
                res += 2;
            }
            if (left[i] - right[i]).abs() == 1 {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the nums array
    let n: usize = lines
        .next()
        .expect("Expected input for n")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Read the elements of the nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for nums array")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse nums element"))
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n, "Input size does not match n");

    // Create an instance of the Solution struct
    let solution = Solution;

    // Call the count_valid_selections method and store the result
    let result = solution.count_valid_selections(nums);

    // Print the result
    println!("{}", result);
}