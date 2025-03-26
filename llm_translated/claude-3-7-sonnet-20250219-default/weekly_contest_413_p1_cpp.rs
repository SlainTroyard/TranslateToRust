use std::io;

struct Solution {}

impl Solution {
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        // Check if the coordinates are on the same color
        // If the sum of differences is even, they're on the same color
        let c1_chars: Vec<char> = coordinate1.chars().collect();
        let c2_chars: Vec<char> = coordinate2.chars().collect();
        
        let diff = (c1_chars[0] as i32 - c2_chars[0] as i32) + (c1_chars[1] as i32 - c2_chars[1] as i32);
        diff % 2 == 0
    }
}

fn main() {
    // Create a buffer for reading input
    let mut input = String::new();
    
    // Read the first coordinate
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read coordinate1");
    let coordinate1 = input.trim().to_string();
    
    // Clear the input buffer and read the second coordinate
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read coordinate2");
    let coordinate2 = input.trim().to_string();
    
    // Create the solution instance and call the function
    let sol = Solution {};
    if sol.check_two_chessboards(&coordinate1, &coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}