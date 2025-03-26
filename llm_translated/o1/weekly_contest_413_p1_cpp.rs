// Problem: Weekly Contest 413 Problem 1
// This Rust program replicates the exact logic of the original C++ code.
// It reads two chessboard coordinates from stdin, checks if they
// have the same color (as per the problem's formula), and prints "true" or "false".

use std::error::Error;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Checks if two chessboard coordinates refer to squares of the same color.
    /// The logic is identical to the original C++ code:
    /// (coordinate1[0] - coordinate2[0] + coordinate1[1] - coordinate2[1]) % 2 == 0
    fn check_two_chessboards(&self, coordinate1: &str, coordinate2: &str) -> bool {
        let c1 = coordinate1.as_bytes();
        let c2 = coordinate2.as_bytes();
        ((c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32)) % 2 == 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read exactly two whitespace-delimited tokens from stdin,
    // replicating the behavior of C++ 'cin >> coordinate1 >> coordinate2'.
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // We'll gather tokens until we have at least two or run out of input.
    for line in stdin.lock().lines() {
        let line = line?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
            if tokens.len() == 2 {
                break;
            }
        }
        if tokens.len() == 2 {
            break;
        }
    }

    // If fewer than 2 tokens are provided, it's an error (like an EOF without enough coordinates).
    if tokens.len() < 2 {
        return Err("Not enough tokens provided".into());
    }

    let coordinate1 = &tokens[0];
    let coordinate2 = &tokens[1];

    let sol = Solution;
    // Perform the exact same check as in the C++ code and print the result.
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}