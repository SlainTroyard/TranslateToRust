// Translated from C++ to Rust for LeetCode Weekly Contest 413 Problem 4.
// This program preserves the exact logic and I/O format of the original C++ code.
//
// INPUT FORMAT (from stdin):
// 1) An integer numsSize
// 2) numsSize integers (may be spread across one or more lines)
// 3) An integer queriesSize
// 4) queriesSize pairs of integers (each pair may be on the same or different lines)
//
// OUTPUT FORMAT (to stdout):
// A sequence of integers (the result) separated by spaces (no extra newline).

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn maximum_subarray_xor(&self, f: &mut Vec<i32>, queries: &Vec<Vec<usize>>) -> Vec<i32> {
        let n = f.len();
        // Prepare a 2D array to store maximum XOR values
        let mut mx = vec![vec![0; n]; n];

        // Build the mx table from the end of the array back to the beginning
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in (i + 1)..n {
                // Cumulative XOR update on f[j]
                f[j] ^= f[j - 1];
                // mx[i][j] is the maximum of: 
                //  1) the newly computed XOR f[j],
                //  2) the best known sub-solution one row down (mx[i+1][j]),
                //  3) or one column to the left (mx[i][j-1]).
                mx[i][j] = std::cmp::max(f[j], std::cmp::max(mx[i + 1][j], mx[i][j - 1]));
            }
        }

        // Answer the queries
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            ans.push(mx[q[0]][q[1]]);
        }
        ans
    }
}

fn main() {
    // Read all tokens from stdin
    let mut tokens = Vec::new();
    {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            for token in line.split_whitespace() {
                tokens.push(token.to_owned());
            }
        }
    }

    // Parse tokens in the same order as the original C++ code
    let mut idx = 0;

    // Read numsSize
    let nums_size = tokens[idx].parse::<usize>().unwrap();
    idx += 1;

    // Read nums
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let val = tokens[idx].parse::<i32>().unwrap();
        idx += 1;
        nums.push(val);
    }

    // Read queriesSize
    let queries_size = tokens[idx].parse::<usize>().unwrap();
    idx += 1;

    // Read queries (pairs of integers)
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let a = tokens[idx].parse::<usize>().unwrap();
        idx += 1;
        let b = tokens[idx].parse::<usize>().unwrap();
        idx += 1;
        queries.push(vec![a, b]);
    }

    // Solve and output results
    let solution = Solution;
    let result = solution.maximum_subarray_xor(&mut nums, &queries);
    for val in result {
        // Print each result followed by a space (no extra newline)
        print!("{} ", val);
    }
}