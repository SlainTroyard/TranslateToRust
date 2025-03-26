// Problem: Weekly Contest 417 Problem 1
// Translated from the original C++ code to idiomatic Rust

use std::io;

/// A struct to encapsulate the solution logic.
struct Solution;

impl Solution {
    /// Returns the k-th character, computed by adding the number of set bits in (k-1)
    /// to the ASCII value of 'a'.
    fn kth_character(&self, k: i64) -> char {
        // Count the number of 1-bits in (k - 1).
        let popcount = (k - 1).count_ones();
        // 'a' plus the number of set bits.
        (b'a' + popcount as u8) as char
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a single integer k from standard input.
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let k: i64 = input.trim().parse()?;

    // Create a solution instance.
    let s = Solution;

    // Print the computed character to standard output.
    println!("{}", s.kth_character(k));

    Ok(())
}