// Problem: Weekly Contest 413 Problem 1
use std::io::{self, BufRead};

/// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1_bytes = coordinate1.as_bytes();
    let c2_bytes = coordinate2.as_bytes();
    
    // Calculate the parity of the sum of differences between coordinates
    // If the result is even (divisible by 2), the squares are of the same color
    ((c1_bytes[0] as i32 - c2_bytes[0] as i32) + (c1_bytes[1] as i32 - c2_bytes[1] as i32)) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the input line and split it into two coordinates
    let input = lines.next().unwrap().unwrap();
    let coordinates: Vec<&str> = input.split_whitespace().collect();
    
    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}