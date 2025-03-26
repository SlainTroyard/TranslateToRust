// Weekly Contest 413 Problem 1 in Rust
//
// This program reads two chessboard coordinates from stdin, checks
// if they are the same color, and prints "true" or "false" accordingly.
// The input/output format matches exactly with the original C code.

use std::error::Error;
use std::io::{self, Read};

/// Checks if the two squares are of the same color on a chessboard.
///
/// Each coordinate is a 2-character string (e.g., "a1").
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1 = coordinate1.as_bytes();
    let c2 = coordinate2.as_bytes();

    // The color check is based on the sum of the ASCII differences of
    // the file (letter) and rank (digit). If the sum is even, they're the same color.
    ((c1[0] as i16 - c2[0] as i16) + (c1[1] as i16 - c2[1] as i16)) % 2 == 0
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read all of stdin into a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input on whitespace to extract two coordinates
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 2 {
        // If there aren't two tokens, the input was insufficient
        return Err("Insufficient input provided".into());
    }

    let coordinate1 = tokens[0];
    let coordinate2 = tokens[1];

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}