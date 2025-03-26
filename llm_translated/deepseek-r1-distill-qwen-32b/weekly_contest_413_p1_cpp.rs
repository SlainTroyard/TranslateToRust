use std::io;

/// Solution to LeetCode Weekly Contest 413 Problem 1
/// Determines if two chessboard coordinates lie on the same color square
struct Solution;

impl Solution {
    /// Checks if two chessboard coordinates are on the same color
    /// # Arguments
    /// * `coordinate1` - A string slice representing the first chessboard coordinate
    /// * `coordinate2` - A string slice representing the second chessboard coordinate
    /// # Returns
    /// * `bool` - true if both coordinates are on the same color, false otherwise
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        let c1_0 = coordinate1.chars().nth(0).unwrap() as i32;
        let c1_1 = coordinate1.chars().nth(1).unwrap() as i32;
        let c2_0 = coordinate2.chars().nth(0).unwrap() as i32;
        let c2_1 = coordinate2.chars().nth(1).unwrap() as i32;
        (c1_0 - c2_0 + c1_1 - c2_1) % 2 == 0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let coords: Vec<&str> = input.trim().split_whitespace().collect();
    
    if coords.len() < 2 {
        println!("false");
        return;
    }
    
    let coordinate1 = coords[0];
    let coordinate2 = coords[1];
    let sol = Solution;
    let result = sol.check_two_chessboards(coordinate1, coordinate2);
    println!("{}", result);
}