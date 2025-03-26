use std::io;

/// Checks if two chessboard squares are of the same color.
/// The chessboard squares are represented by their coordinates (e.g., "a1", "b2").
/// The function returns true if both squares are the same color, false otherwise.
fn check_two_chessboards(coord1: &str, coord2: &str) -> bool {
    let c1 = coord1.as_bytes();
    let c2 = coord2.as_bytes();
    // Calculate the sum of the ASCII values of the first two characters for each coordinate
    let sum1 = c1[0] + c1[1];
    let sum2 = c2[0] + c2[1];
    // Check if the sums have the same parity (both even or both odd)
    (sum1 % 2) == (sum2 % 2)
}

fn main() {
    // Read the input line containing two coordinates
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Split the input into two coordinates, using whitespace as the delimiter
    let coords: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Check if exactly two coordinates were provided
    if coords.len() < 2 {
        println!("false");
        return;
    }
    
    let coord1 = coords[0];
    let coord2 = coords[1];
    
    // Ensure each coordinate has at least two characters
    if coord1.len() < 2 || coord2.len() < 2 {
        println!("false");
        return;
    }
    
    // Check if the two squares are of the same color and print the result
    let result = check_two_chessboards(coord1, coord2);
    println!("{}", result);
}