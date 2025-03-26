// Problem: Weekly Contest 413 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Check if two chessboard coordinates are on the same color
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // Convert the characters to their ASCII values and compute the difference
        let diff = (coordinate1.as_bytes()[0] as i32 - coordinate2.as_bytes()[0] as i32)
            + (coordinate1.as_bytes()[1] as i32 - coordinate2.as_bytes()[1] as i32);
        // Check if the sum of the differences is even
        diff % 2 == 0
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Ensure we have exactly two coordinates
    if input.len() != 2 {
        eprintln!("Expected exactly two coordinates");
        return;
    }
    
    let coordinate1 = input[0];
    let coordinate2 = input[1];
    
    // Create an instance of Solution and call the check function
    let sol = Solution;
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}