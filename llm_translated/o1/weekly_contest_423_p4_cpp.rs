// Equivalent Rust solution for "Weekly Contest 423 Problem 4"
// -----------------------------------------------------------
// This program reads a string (s) and an integer (k) from stdin,
// then computes and prints the result using the same logic as
// the original C++ solution.
//
// To run:
//   1. Save this file as main.rs
//   2. Compile: rustc main.rs
//   3. Run:     ./main
//
// Expected I/O behavior matches the C++ code exactly.

use std::error::Error;
use std::io::{self, BufRead};

// A structure to hold state during our DP (dynamic programming) solution.
struct Solution {
    // cnt[i] holds how many times we need to repeatedly take popcount
    // to reach a value < k in the original code.
    cnt: Vec<i32>,
    // The threshold k, read from input.
    k: i32,
    // A 3D DP table storing results to avoid recomputation.
//  dp[i][tight][set_bits] represents the number of valid ways
//  given parsing up to index i, whether we are tight (matching
//  the prefix exactly up to that position), and how many set
//  bits we had so far.
    dp: [[[i64; 801]; 2]; 801],
}

// We define our modulo constant as given in the problem
const MOD: i64 = 1_000_000_007;

impl Solution {
    // Constructs a new solution object with default values
    fn new() -> Self {
        Self {
            cnt: vec![0; 801],
            k: 0,
            dp: [[[ -1; 801 ]; 2]; 801],
        }
    }

    // Recursive DP function akin to "solve" in the original C++ code.
    //
    // s: the binary string we read from input
    // i: current index in the string
    // tight: whether we're still matching exactly up to index i
    // set_bits: how many set bits seen so far
    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: usize) -> i64 {
        // If we've reached the end of the string:
        if i == s.len() {
            // Return 0 if still 'tight' or no set bits at all.
            // Otherwise, return 1 if the repeated popcount of set_bits is < k, else 0.
            if tight || set_bits == 0 {
                return 0;
            } else {
                return if self.cnt[set_bits] < self.k { 1 } else { 0 };
            }
        }

        // If we already computed dp[i][tight][set_bits], return it.
        let dp_val = self.dp[i][tight as usize][set_bits];
        if dp_val != -1 {
            return dp_val;
        }

        let res: i64;
        // If we are still 'tight', we must check the current bit:
        if tight {
            if s.as_bytes()[i] == b'0' {
                // If current bit is '0', we must stay tight and don't increment set_bits
                res = self.solve(s, i + 1, true, set_bits);
            } else {
                // If current bit is '1':
                // 1) stay tight and increment set_bits
                // 2) flip tight to false and don't increment set_bits
                let mut temp = self.solve(s, i + 1, true, set_bits + 1);
                temp = (temp + self.solve(s, i + 1, false, set_bits)) % MOD;
                res = temp;
            }
        } else {
            // If not tight, we can freely pick '0' or '1':
            // 1) pick '1' => increment set_bits
            // 2) pick '0' => do not increment set_bits
            let mut temp = self.solve(s, i + 1, false, set_bits + 1);
            temp = (temp + self.solve(s, i + 1, false, set_bits)) % MOD;
            res = temp;
        }

        // Store result in DP table and return
        self.dp[i][tight as usize][set_bits] = res % MOD;
        res
    }

    // This function sets up the DP arrays and calls solve() similar to
    // "countKReducibleNumbers" in the original C++ code.
    fn count_k_reducible_numbers(&mut self, s: &str, k: i32) -> i64 {
        self.k = k;
        // Reset cnt array and dp table
        for x in self.cnt.iter_mut() {
            *x = 0;
        }
        for i in 0..801 {
            for j in 0..2 {
                for m in 0..801 {
                    self.dp[i][j][m] = -1;
                }
            }
        }
        // Precompute how many times we reduce via popcount until < k
        // (Similar logic to: cnt[i] = 1 + cnt[popcount(i)] in the C++ code)
        for i in 2..=800 {
            let set_bits = (i as u32).count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        // Kick off the recursion
        self.solve(s, 0, true, 0) % MOD
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read tokens from stdin (s, then k), matching the C++ code's std::cin >> s >> k
    let mut tokens = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
        if tokens.len() >= 2 {
            break;
        }
    }

    // Extract the string and integer k
    let s_input = &tokens[0];
    let k_input: i32 = tokens[1].parse()?;

    // Create our solution object and compute the result
    let mut sol = Solution::new();
    let result = sol.count_k_reducible_numbers(s_input, k_input);

    // Print the result (one line), exactly like the C++ code
    println!("{}", result);

    Ok(())
}