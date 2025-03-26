// Problem: Weekly Contest 413 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Check if two chessboard coordinates are on the same color
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // Calculate the difference in file (column) and rank (row) positions
        let file_diff = (coordinate1.chars().nth(0).unwrap() as u8) - (coordinate2.chars().nth(0).unwrap() as u8);
        let rank_diff = (coordinate1.chars().nth(1).unwrap() as u8) - (coordinate2.chars().nth(1).unwrap() as u8);

        // Check if the sum of differences is even
        (file_diff + rank_diff) % 2 == 0
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

    // Create an instance of the Solution struct
    let sol = Solution;

    // Check if the two coordinates are on the same color
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}