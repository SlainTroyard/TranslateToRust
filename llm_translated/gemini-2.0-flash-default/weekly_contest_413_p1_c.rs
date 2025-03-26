use std::io;

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1_0 = coordinate1.chars().nth(0).unwrap() as i32;
    let c1_1 = coordinate1.chars().nth(1).unwrap() as i32;
    let c2_0 = coordinate2.chars().nth(0).unwrap() as i32;
    let c2_1 = coordinate2.chars().nth(1).unwrap() as i32;

    (c1_0 - c2_0 + c1_1 - c2_1) % 2 == 0
}

fn main() {
    // Read the coordinates of the two squares
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
    
    if coordinates.len() != 2 {
        eprintln!("Error: Expected two coordinates.");
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