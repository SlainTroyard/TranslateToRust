use std::io::{self, BufRead};

// Function to check if the two squares are of the same color
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1 = coordinate1.as_bytes();
    let c2 = coordinate2.as_bytes();
    (c1[0] as i32 - c2[0] as i32 + c1[1] as i32 - c2[1] as i32) % 2 == 0
}

fn main() -> io::Result<()> {
    // Read the coordinates of the two squares
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing both coordinates
    let input = lines.next().unwrap()?;
    let mut parts = input.split_whitespace();

    let coordinate1 = parts.next().unwrap();
    let coordinate2 = parts.next().unwrap();

    // Check if the two squares are of the same color
    if check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}