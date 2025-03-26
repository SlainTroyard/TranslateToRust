// Problem: Weekly Contest 413 Problem 1
use std::io;

struct Solution;

impl Solution {
    // Function to check if two chessboard coordinates are on the same color
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // Calculate the difference in ASCII values of the characters
        // and check if the sum of differences is even
        let diff = (coordinate1.chars().nth(0).unwrap() as i32 - coordinate2.chars().nth(0).unwrap() as i32)
            + (coordinate1.chars().nth(1).unwrap() as i32 - coordinate2.chars().nth(1).unwrap() as i32);
        diff % 2 == 0
    }
}

fn main() {
    // Create a new input reader
    let mut input = String::new();

    // Read the input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split the input into two coordinates
    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
    if coordinates.len() != 2 {
        eprintln!("Invalid input format. Expected two coordinates.");
        return;
    }

    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];

    // Create an instance of the solution
    let sol = Solution;

    // Check if the two coordinates are on the same color
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}