use std::io;

/// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1_bytes = coordinate1.as_bytes();
    let c2_bytes = coordinate2.as_bytes();
    
    // Calculate the sum of differences of coordinates
    // If the sum is even, the squares have the same color
    ((c1_bytes[0] as i32 - c2_bytes[0] as i32) + (c1_bytes[1] as i32 - c2_bytes[1] as i32)) % 2 == 0
}

fn main() {
    // Read the input line
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Split the input into two coordinates
    let coordinates: Vec<&str> = input
        .trim()
        .split_whitespace()
        .collect();
    
    // Extract the two coordinates
    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];
    
    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}