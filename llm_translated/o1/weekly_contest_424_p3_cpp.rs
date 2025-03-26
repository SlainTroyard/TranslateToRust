// Problem: Weekly Contest 424 Problem 3
// Translated from C++ to Rust
//
// This program reads an integer n, then reads n integers into a vector "nums".
// Then it reads an integer m, followed by m queries (three integers per query).
// It then computes the result of the minZeroArray function and prints the result.
//
// The logic and I/O format are preserved exactly from the original C++ solution.

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Translated version of the original C++ minZeroArray function.
    /// This function uses a difference array (d) to track changes in the "nums" array.
    /// Then it processes queries that modify segments of the array.
    /// The function returns the first query index (1-based) after which the "acc" (accumulator)
    /// can no longer stay above 0, indicating the array is "zeroed" in the problem's sense.
    /// If the array is already in that state at the start, it returns 0.
    /// If no such query exists, it returns -1.
    fn min_zero_array(&self, nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        // Create a difference array d of length n+1. 
        // This allows us to apply range updates without repeated iteration.
        let mut d = vec![0; n + 1];

        // Build the difference array from nums
        if n > 0 {
            d[0] = nums[0];
        }
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        // acc tracks the cumulative sum as we move through d.
        // cur is the current index in d.
        let mut acc = 0;
        let mut cur = -1_i32;

        // Move 'cur' forward while the cumulative sum <= 0
        while acc <= 0 && (cur as usize) < n {
            cur += 1;
            acc += d[cur as usize];
        }

        // If we've reached the end, return 0 (indicating no queries needed)
        if cur as usize == n {
            return 0;
        }

        // Process each query
        let m = queries.len();
        for i in 0..m {
            // Each query has [l, r, val]
            let l = queries[i][0];
            let r = queries[i][1];
            let val = queries[i][2];

            // Apply the range update to the difference array
            // d[r+1] += val; d[l] -= val;
            // Safe because we created d with size n+1 and r is at most n-1 (per problem constraints).
            d[(r + 1) as usize] += val;
            d[l as usize] -= val;

            // If our current index 'cur' is within [l, r], we need to adjust 'acc' accordingly
            if cur >= l && cur <= r {
                acc -= val;
                // Move 'cur' forward while cumulative sum <= 0
                while acc <= 0 && (cur as usize) < n {
                    cur += 1;
                    acc += d[cur as usize];
                }
                // If we've reached 'n', return the current query index + 1 (1-based)
                if cur as usize == n {
                    return i as i32 + 1;
                }
            }
        }

        // If we finish all queries without returning, the answer is -1
        -1
    }
}

fn main() {
    // Prepare to read from stdin
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all tokens (integers) from stdin (same as repeated cin >> in C++)
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        for word in line.split_whitespace() {
            tokens.push(word.parse::<i32>().expect("Failed to parse integer"));
        }
    }

    // Use an index to walk through the collected tokens
    let mut index = 0;

    // Read the size of nums
    let n = tokens[index] as usize;
    index += 1;

    // Read the nums array
    let mut nums = vec![0; n];
    for i in 0..n {
        nums[i] = tokens[index];
        index += 1;
    }

    // Read the number of queries
    let m = tokens[index] as usize;
    index += 1;

    // Read the queries
    let mut queries = vec![vec![0; 3]; m];
    for i in 0..m {
        queries[i][0] = tokens[index];
        index += 1;
        queries[i][1] = tokens[index];
        index += 1;
        queries[i][2] = tokens[index];
        index += 1;
    }

    // Create a Solution instance and compute the answer
    let sol = Solution;
    let ans = sol.min_zero_array(&nums, &queries);

    // Print the result (same format as original C++ code)
    println!("{}", ans);
}