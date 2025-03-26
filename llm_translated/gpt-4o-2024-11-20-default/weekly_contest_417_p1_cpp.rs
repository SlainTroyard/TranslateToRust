// Problem: Weekly Contest 417 Problem 1

use std::io;

struct Solution;

impl Solution {
    /// Computes the kth character as described in the original CPP code logic.
    fn kth_character(k: i64) -> char {
        // 'a' is the base character, and we add the popcount of (k - 1).
        'a' + (k - 1).count_ones() as u8
    }
}

fn main() {
    // Step 1: Read the input value for `k` from stdin.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: i64 = input.trim().parse().expect("Failed to parse input as integer");

    // Step 2: Create an instance of the Solution struct and compute the result.
    let solution = Solution;
    let result = solution.kth_character(k);

    // Step 3: Print the result to stdout.
    println!("{}", result);
}