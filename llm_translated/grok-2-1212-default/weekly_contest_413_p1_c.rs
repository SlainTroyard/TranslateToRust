use std::io::{self, BufRead};

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let coord1 = coordinate1.as_bytes();
    let coord2 = coordinate2.as_bytes();
    (coord1[0] as i32 - coord2[0] as i32 + coord1[1] as i32 - coord2[1] as i32) % 2 == 0
}

fn main() -> io::Result<()> {
    // Read the coordinates of the two squares
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let coordinate1 = lines.next().unwrap()?.trim().to_string();
    let coordinate2 = lines.next().unwrap()?.trim().to_string();

    // Check if the two squares are of the same color
    if check_two_chessboards(&coordinate1, &coordinate2) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}