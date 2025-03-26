// Problem: Weekly Contest 413 Problem 1

use std::io;

/// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let (c1_col, c1_row) = (coordinate1.chars().next().unwrap(), coordinate1.chars().nth(1).unwrap());
    let (c2_col, c2_row) = (coordinate2.chars().next().unwrap(), coordinate2.chars().nth(1).unwrap());

    // Convert column characters to 0-based index (A=0, B=1, ...)
    let c1_col_index = (c1_col as u8 - b'A') as i32;
    let c2_col_index = (c2_col as u8 - b'A') as i32;

    // Convert row characters to 0-based index (1=0, 2=1, ...)
    let c1_row_index = (c1_row as u8 - b'1') as i32;
    let c2_row_index = (c2_row as u8 - b'1') as i32;

    // Check if the sum of differences is even
    (c1_col_index - c2_col_index + c1_row_index - c2_row_index) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let (coordinate1, coordinate2) = (input[0], input[1]);

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}