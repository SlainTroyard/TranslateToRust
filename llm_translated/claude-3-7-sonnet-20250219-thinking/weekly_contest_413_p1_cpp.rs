use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Check if two chess positions are on the same color
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // In a chessboard, positions have the same color if the sum of differences
        // between their coordinates is even
        let c1: Vec<char> = coordinate1.chars().collect();
        let c2: Vec<char> = coordinate2.chars().collect();
        
        ((c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32)) % 2 == 0
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read input");
    
    // Split the input by whitespace to get the two coordinates
    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Extract the two coordinates
    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];
    
    // Create an instance of Solution and check
    let sol = Solution;
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}