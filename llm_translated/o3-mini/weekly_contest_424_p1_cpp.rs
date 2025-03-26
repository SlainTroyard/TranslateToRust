use std::error::Error;
use std::io::{self, Read};

/// Struct to encapsulate our solution logic.
struct Solution;

impl Solution {
    /// Count valid selections based on the problem logic.
    ///
    /// This function expects a reference to a vector of integers (`nums`)
    /// and returns an integer representing the result.
    fn count_valid_selections(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut res = 0;
        // Initialize prefix sum vectors for left and right.
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        // Compute prefix sums from the left.
        // left[i] = sum of all numbers in nums[0..i]
        for i in 1..n {
            left[i] = left[i - 1] + nums[i - 1];
        }

        // Compute prefix sums from the right.
        // right[i] = sum of all numbers in nums[i+1..n]
        for i in 1..n {
            right[n - i - 1] = right[n - i] + nums[n - i];
        }

        // Iterate through each element.
        // For every element that is zero, check for conditions:
        // 1. If left sum equals right sum, add 2.
        // 2. If the absolute difference between left and right is 1, add 1.
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

/// Helper function to read all input from stdin and split into tokens.
fn read_tokens() -> Result<Vec<String>, Box<dyn Error>> {
    // Read the whole input as a single string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // Split input by whitespace and collect into a vector.
    Ok(input.split_whitespace().map(|s| s.to_string()).collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read all tokens from standard input.
    let tokens = read_tokens()?;
    let mut iter = tokens.into_iter();

    // The first token is the number of elements.
    let n: usize = iter
        .next()
        .ok_or("Expected number of elements")?
        .parse()?;
    
    // Read the next n integers into a vector.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter
            .next()
            .ok_or("Expected an integer value")?
            .parse()?;
        nums.push(num);
    }

    // Create a Solution instance and compute the result using our method.
    let result = Solution::count_valid_selections(&nums);

    // Print the result to stdout (exactly as the original C++ code).
    println!("{}", result);
    Ok(())
}