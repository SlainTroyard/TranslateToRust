use std::io;

/// Checks if two chessboard squares are of the same color.
///
/// # Arguments
/// * `coordinate1` - The first coordinate (e.g., "a1")
/// * `coordinate2` - The second coordinate (e.g., "h8")
///
/// # Returns
/// * `true` if both squares are the same color, `false` otherwise
fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
    let c1 = coordinate1.as_bytes();
    let c2 = coordinate2.as_bytes();
    // Calculate the difference between the first characters and the second characters
    let diff = (c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32);
    diff % 2 == 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    // Ensure exactly two coordinates are provided, mirroring C's behavior
    let (coordinate1, coordinate2) = if parts.len() >= 2 {
        (parts[0], parts[1])
    } else {
        // If fewer than two parts, take the first available and an empty string (matches C's undefined behavior)
        (parts.get(0).cloned().unwrap_or(""), "")
    };
    let result = check_two_chessboards(coordinate1, coordinate2);
    println!("{}", result);
}