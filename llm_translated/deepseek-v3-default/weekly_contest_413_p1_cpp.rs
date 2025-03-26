use std::io;

struct Solution;

impl Solution {
    // Function to check if two chessboard coordinates are of the same color
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // Calculate the sum of the differences in the x and y coordinates
        let x_diff = coordinate1.chars().nth(0).unwrap() as i32 - coordinate2.chars().nth(0).unwrap() as i32;
        let y_diff = coordinate1.chars().nth(1).unwrap() as i32 - coordinate2.chars().nth(1).unwrap() as i32;
        (x_diff + y_diff) % 2 == 0
    }
}

fn main() {
    let mut input = String::new();
    
    // Read the first coordinate from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let coordinate1 = input.trim().to_string();
    
    // Clear the input buffer
    input.clear();
    
    // Read the second coordinate from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let coordinate2 = input.trim().to_string();
    
    // Create an instance of Solution
    let sol = Solution;
    
    // Check if the two coordinates are of the same color and print the result
    if sol.check_two_chessboards(&coordinate1, &coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}