// Problem: Weekly Contest 413 Problem 1
// Rust translation of the C program

use std::io::{self, Write};

/// Function to check if two chessboard coordinates are of the same color
/// The function takes two strings as inputs (coordinates like "a1" and "h8") and returns a boolean.
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    // Convert the characters of the coordinates into their corresponding numeric representations.
    let coord1_bytes = coordinate1.as_bytes();
    let coord2_bytes = coordinate2.as_bytes();

    // Chessboard coloring logic: determine if both coordinates have the same parity
    // Note: ASCII values of 'a'-'h' or 'A'-'H' and '1'-'8' are used here.
    ((coord1_bytes[0] as i32 - coord2_bytes[0] as i32)
        + (coord1_bytes[1] as i32 - coord2_bytes[1] as i32))
        % 2
        == 0
}

fn main() {
    // Create a buffer for reading input
    let mut input = String::new();

    // Read a line of input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into two parts (coordinates), trim whitespace, and assign to variables
    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();

    // Input validation to ensure we received exactly two coordinate strings
    if coordinates.len() != 2 {
        eprintln!("Error: Please provide exactly 2 chessboard coordinates.");
        std::process::exit(1);
    }

    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];

    // Check if the two squares have the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}