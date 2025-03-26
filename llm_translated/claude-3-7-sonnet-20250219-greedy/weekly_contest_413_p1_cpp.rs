use std::io;

struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        // Check if both coordinates are on the same color
        // On a chessboard, squares have alternating colors
        // If the sum of differences in coordinates is even, they're on the same color
        let c1: Vec<char> = coordinate1.chars().collect();
        let c2: Vec<char> = coordinate2.chars().collect();
        
        ((c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32)) % 2 == 0
    }
}

fn main() {
    // Read input coordinates
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read coordinate1");
    let coordinate1 = input.trim().to_string();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read coordinate2");
    let coordinate2 = input.trim().to_string();
    
    // Create solution instance and check result
    let sol = Solution;
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}