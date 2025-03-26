use std::io;

// Define a struct to encapsulate the solution logic
struct Solution;

impl Solution {
    // Method to check if two chessboard coordinates are of the same color
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // Calculate the sum of the differences in the x and y coordinates
        let x_diff = coordinate1.chars().nth(0).unwrap() as i32 - coordinate2.chars().nth(0).unwrap() as i32;
        let y_diff = coordinate1.chars().nth(1).unwrap() as i32 - coordinate2.chars().nth(1).unwrap() as i32;
        // Check if the sum is even
        (x_diff + y_diff) % 2 == 0
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Ensure exactly two coordinates are provided
    if coordinates.len() != 2 {
        eprintln!("Please provide exactly two coordinates.");
        return;
    }
    
    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];
    
    // Create an instance of Solution and check the coordinates
    let result = Solution::check_two_chessboards(coordinate1, coordinate2);
    
    // Print the result in the required format
    if result {
        println!("true");
    } else {
        println!("false");
    }
}