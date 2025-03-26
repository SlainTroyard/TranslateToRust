use std::io::{self, Read};

/// Checks if two chessboard coordinates are the same color using the same logic
/// as the original C code. The coordinates must be 2-character ASCII strings.
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1 = coordinate1.as_bytes();
    let c2 = coordinate2.as_bytes();
    let diff = (c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32);
    diff % 2 == 0
}

fn main() {
    // Read all input at once to handle any whitespace separation including newlines
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Split into whitespace-separated tokens preserving original parsing logic
    let mut tokens = input.split_whitespace();
    let coord1 = tokens.next().expect("Need first coordinate");
    let coord2 = tokens.next().expect("Need second coordinate");

    // Ensure exactly 2-character format like "a1" as in original code
    assert!(coord1.len() == 2 && coord2.len() == 2, "Invalid coordinate format");

    // Execute core logic and print result with original format
    println!("{}", check_two_chessboards(coord1, coord2));
}