use std::io::{self, Read};

/// Checks if two chess coordinates are on the same color square using the same logic as the original C++ code.
/// The condition is derived from the sum of character differences modulo 2.
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    // Safely extract ASCII values of characters, panics on invalid coordinates
    let c1: Vec<char> = coordinate1.chars().collect();
    let c2: Vec<char> = coordinate2.chars().collect();
    
    // Calculate sum of differences using integer arithmetic to match C++ behavior
    let sum_diff = (c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32);
    sum_diff % 2 == 0
}

fn main() {
    // Read all input at once to handle any whitespace/newline separation
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    // Split into whitespace-separated tokens exactly like C++ cin>>
    let mut tokens = input.split_whitespace();
    let coordinate1 = tokens.next().expect("First coordinate missing");
    let coordinate2 = tokens.next().expect("Second coordinate missing");

    // Preserve original output format with newline
    println!("{}", check_two_chessboards(coordinate1, coordinate2));
}