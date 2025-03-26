use std::error::Error;
use std::io::{self, Read};

// Define the Solution struct.
struct Solution;

impl Solution {
    // This function checks if two chessboards (given by their coordinates)
    // have the same color. It replicates the logic from the C++ solution.
    fn check_two_chessboards(&self, coordinate1: &str, coordinate2: &str) -> bool {
        // Convert the strings to bytes to get direct access to the character values.
        let b1 = coordinate1.as_bytes();
        let b2 = coordinate2.as_bytes();

        // Ensure that both coordinates have at least 2 characters.
        // (This mirrors the assumptions made in the C++ code.)
        if b1.len() < 2 || b2.len() < 2 {
            // If not, we simply return false.
            return false;
        }
        
        // Compute the difference as in the original C++ code.
        let diff = (b1[0] as i32 - b2[0] as i32) + (b1[1] as i32 - b2[1] as i32);
        diff % 2 == 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input by whitespace to cover both multiple lines and multiple values per line.
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Ensure there are at least two tokens; otherwise, return an error.
    if tokens.len() < 2 {
        return Err("Not enough input tokens. Expected two coordinates.".into());
    }

    // The first two tokens are the coordinates.
    let coordinate1 = tokens[0];
    let coordinate2 = tokens[1];

    // Create a Solution instance and use it to check the chessboards.
    let sol = Solution;
    let result = sol.check_two_chessboards(coordinate1, coordinate2);

    // Output must exactly match the original C++ code.
    println!("{}", if result { "true" } else { "false" });

    Ok(())
}