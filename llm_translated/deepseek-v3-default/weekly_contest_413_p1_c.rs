use std::io::{self, Write};

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    // Calculate the sum of the differences in the x and y coordinates
    let diff_x = coordinate1.chars().nth(0).unwrap() as i32 - coordinate2.chars().nth(0).unwrap() as i32;
    let diff_y = coordinate1.chars().nth(1).unwrap() as i32 - coordinate2.chars().nth(1).unwrap() as i32;
    
    // Check if the sum is even
    (diff_x + diff_y) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Split the input into two coordinates
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