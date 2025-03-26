use std::error::Error;
use std::io::{self, BufRead};

/// Check if the two chessboard squares have the same color.
///
/// The color is determined by comparing the sum of differences between
/// the ASCII values of the corresponding coordinate characters.
/// Returns `true` if the sum of differences is even.
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    // Convert the coordinate strings to bytes for ASCII arithmetic.
    let bytes1 = coordinate1.as_bytes();
    let bytes2 = coordinate2.as_bytes();

    // Perform the arithmetic on the first two characters of each coordinate.
    // This mirrors the logic of:
    // (coordinate1[0] - coordinate2[0] + coordinate1[1] - coordinate2[1]) % 2 == 0
    let diff = (bytes1[0] as i32 - bytes2[0] as i32) + (bytes1[1] as i32 - bytes2[1] as i32);

    // Check if the result is even.
    diff % 2 == 0
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a BufReader to handle input from stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Read a single line from stdin.
    let mut input = String::new();
    reader.read_line(&mut input)?;
    
    // Parse the line's whitespace-separated tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 2 {
        return Err("Expected at least two coordinates as input.".into());
    }

    // According to the original C code, we only take the first two tokens.
    let coordinate1 = tokens[0];
    let coordinate2 = tokens[1];

    // Check if the two squares are of the same color.
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}