// Problem: Weekly Contest 427 Problem 3

use std::collections::BTreeMap;
use std::cmp::{max, min};
use std::io::{self, Read};

/// A struct representing our solution, matching the C++ 'Solution' class.
struct Solution;

impl Solution {
    /// Returns the maximum subarray sum for the given vector `v` with subarray size multiples of `k`.
    /// This logic is directly translated from the provided C++ code.
    fn max_subarray_sum(&self, v: &[i32], k: i32) -> i64 {
        let mut m = BTreeMap::new();
        let mut ans = i64::MIN;
        let mut sm = 0_i64;

        for (i, &val) in v.iter().enumerate() {
            // Update the running sum
            sm += val as i64;

            // Current size of the subarray
            let cur_sz = (i + 1) as i32;

            // If the current subarray size is divisible by k, update the answer
            if cur_sz % k == 0 {
                ans = max(ans, sm);
            }

            // Check if we have a previous sum for the same remainder
            let remainder = cur_sz % k;
            if let Some(prev_sm) = m.get_mut(&remainder) {
                ans = max(ans, sm - *prev_sm);
                *prev_sm = min(*prev_sm, sm);
            } else {
                m.insert(remainder, sm);
            }
        }

        ans
    }
}

fn main() -> io::Result<()> {
    // Read entire standard input into a string for tokenized parsing
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str)?;

    // Split the input by whitespace for sequential reading
    let mut tokens = input_str.split_whitespace();

    // Read n (array size)
    let n: usize = tokens
        .next()
        .expect("Failed to read n")
        .parse()
        .expect("Failed to parse n as a number");

    // Read k
    let k: i32 = tokens
        .next()
        .expect("Failed to read k")
        .parse()
        .expect("Failed to parse k as a number");

    // Read the array of integers
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let x = tokens
            .next()
            .expect("Insufficient input for array elements")
            .parse::<i32>()
            .expect("Failed to parse array element");
        nums.push(x);
    }

    // Create the solution object
    let solution = Solution;

    // Calculate the result
    let result = solution.max_subarray_sum(&nums, k);

    // Print the result to stdout (matching the original C++ output format)
    println!("{}", result);

    Ok(())
}