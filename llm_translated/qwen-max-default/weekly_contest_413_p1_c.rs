// Problem: Weekly Contest 413 Problem 1
use std::io::{self, BufRead};

/// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let (c1, r1) = (coordinate1.chars().next().unwrap(), coordinate1.chars().nth(1).unwrap());
    let (c2, r2) = (coordinate2.chars().next().unwrap(), coordinate2.chars().nth(1).unwrap());

    // Convert characters to their ASCII values and check the parity
    (c1 as u8 - c2 as u8 + r1 as u8 - r2 as u8) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Remove any trailing newline or whitespace

    // Split the input into the two coordinates
    let coordinates: Vec<&str> = input.split_whitespace().collect();
    if coordinates.len() != 2 {
        eprintln!("Invalid input format. Expected two coordinates.");
        return;
    }

    let (coordinate1, coordinate2) = (coordinates[0], coordinates[1]);

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}