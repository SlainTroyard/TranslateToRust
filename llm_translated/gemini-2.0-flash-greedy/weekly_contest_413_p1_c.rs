use std::io;

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1_chars: Vec<char> = coordinate1.chars().collect();
    let c2_chars: Vec<char> = coordinate2.chars().collect();

    let col1 = c1_chars[0] as i32 - 'a' as i32;
    let row1 = c1_chars[1] as i32 - '1' as i32;
    let col2 = c2_chars[0] as i32 - 'a' as i32;
    let row2 = c2_chars[1] as i32 - '1' as i32;

    (col1 - col2 + row1 - row2) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}