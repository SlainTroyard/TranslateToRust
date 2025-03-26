use std::io::{self, BufRead}; // For reading input
use std::cmp::Ordering;

/// Helper struct to encapsulate solution logic
struct Solution;

impl Solution {
    /// Function to count valid selections
    fn count_valid_selections(nums: Vec<i32>) -> i32 {
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
            match left[i].cmp(&right[i]) {
                Ordering::Equal => res += 2,
                Ordering::Greater | Ordering::Less if (left[i] - right[i]).abs() == 1 => res += 1,
                _ => (),
            }
        }

        res
    }
}

fn main() {
    // Read input using stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse first line for the number of elements
    let n: usize = lines
        .next()
        .expect("Expected a line containing the number of elements")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse the number of elements");

    // Parse the second line for the elements of the nums array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected a line containing the elements")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse an element"))
        .collect();

    // Ensure the input array length matches the given number of elements
    assert_eq!(nums.len(), n, "Input array length does not match the specified number of elements");

    // Create an instance of the solution, call the method, and print the result
    let solution = Solution;
    let result = solution.count_valid_selections(nums);

    // Output the result to stdout
    println!("{}", result);
}