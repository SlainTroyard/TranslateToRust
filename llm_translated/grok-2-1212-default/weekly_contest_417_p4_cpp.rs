use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and operationSize
    let first_line: Vec<i64> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let k = first_line[0];
    let operation_size = first_line[1] as usize;

    // Read operations
    let operations: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = kth_character(k, &operations);
    println!("{}", result);

    Ok(())
}

/// Calculates the kth character based on the given operations.
///
/// # Arguments
///
/// * `k` - The position of the character to find (1-indexed).
/// * `operations` - A vector of integers representing the operations.
///
/// # Returns
///
/// The kth character as a char.
fn kth_character(k: i64, operations: &[i32]) -> char {
    let mut k = k - 1; // Convert to 0-indexed
    let mut inc = 0;

    // Calculate the number of bits needed to represent k
    let mut i = 63; // Start from the highest possible bit for i64
    while i >= 0 {
        if k & (1 << i) != 0 {
            inc += operations[i as usize];
        }
        i -= 1;
    }

    // Calculate the final character
    let result = 'a' as u8 + (inc % 26) as u8;
    result as char
}