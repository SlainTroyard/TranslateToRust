use std::io;

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let coord1_bytes = coordinate1.as_bytes();
    let coord2_bytes = coordinate2.as_bytes();
    
    // Calculate if squares are the same color
    // In chess, if the sum of differences in row and column is even, the squares are of the same color
    ((coord1_bytes[0] as i32 - coord2_bytes[0] as i32) + (coord1_bytes[1] as i32 - coord2_bytes[1] as i32)) % 2 == 0
}

fn main() {
    // Create buffer for input
    let mut input = String::new();
    
    // Read the coordinates from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input by whitespace to get the two coordinates
    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
    
    if coordinates.len() != 2 {
        eprintln!("Expected two coordinates, got {}", coordinates.len());
        return;
    }
    
    // Extract the two coordinates
    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];
    
    // Check if the two squares are of the same color and print result
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}