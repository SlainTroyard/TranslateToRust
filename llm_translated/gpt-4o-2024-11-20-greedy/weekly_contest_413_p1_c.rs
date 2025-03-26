use std::io;

/// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1 = coordinate1.as_bytes();
    let c2 = coordinate2.as_bytes();
    (c1[0] as i32 - c2[0] as i32 + c1[1] as i32 - c2[1] as i32) % 2 == 0
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the coordinates of the two squares
    stdin.read_line(&mut input).expect("Failed to read input");
    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();

    // Ensure we have exactly two coordinates
    if coordinates.len() != 2 {
        eprintln!("Invalid input format. Expected two coordinates.");
        return;
    }

    let coordinate1 = coordinates[0];
    let coordinate2 = coordinates[1];

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}