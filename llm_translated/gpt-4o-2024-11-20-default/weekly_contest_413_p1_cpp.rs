// Problem: Weekly Contest 413 Problem 1

use std::io;

struct Solution;

impl Solution {
    /// Check if two chessboard squares have the same color.
    /// In chessboards, alternating squares are of the same color, hence the sum
    /// of character differences modulo 2 determines if both are on the same color.
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        let c1 = coordinate1.as_bytes();
        let c2 = coordinate2.as_bytes();
        
        // Subtract ASCII values and calculate modulo 2 to check color similarity
        (c1[0] as i32 - c2[0] as i32 + c1[1] as i32 - c2[1] as i32) % 2 == 0
    }
}

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split input into two coordinates (assuming input format matches CPP's behavior)
    let mut coordinates = input.trim().split_whitespace();
    
    let coordinate1 = coordinates.next().expect("Missing first coordinate");
    let coordinate2 = coordinates.next().expect("Missing second coordinate");
    
    // Create an instance of Solution and check the result
    let sol = Solution;
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}