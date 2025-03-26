// Problem: Weekly Contest 413 Problem 1 in Rust

use std::io;

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    // Convert coordinates to bytes for easier arithmetic
    let coord1_bytes = coordinate1.as_bytes();
    let coord2_bytes = coordinate2.as_bytes();

    // Calculate the difference in column and row indices and check parity
    (coord1_bytes[0] - coord2_bytes[0] + coord1_bytes[1] - coord2_bytes[1]) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares from stdin
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");

    // Split the input line by whitespace to get two coordinates
    let coordinates: Vec<&str> = input_line.trim().split_whitespace().collect();

    // Ensure we have exactly two coordinates
    if coordinates.len() != 2 {
        eprintln!("Error: Expected two coordinates as input");
        return;
    }

    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];

    // Check if the coordinates are valid (optional, but good practice)
    if coordinate1.len() != 2 || coordinate2.len() != 2 {
        eprintln!("Error: Invalid coordinate format");
        return;
    }

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}