use std::io;

/// Checks if a number string is balanced by ensuring the alternating sum of digits is zero.
///
/// The algorithm works by iterating over each digit, updating a total value by adding the negative
/// of the digit's numeric value, then negating the total each step. If the final total is zero,
/// the number is balanced (sum of even-indexed digits equals sum of odd-indexed digits).
fn is_balanced(s: &str) -> bool {
    let mut total = 0;
    for c in s.chars() {
        // Convert char to numeric value and subtract from '0' to get negative digit value
        total += '0' as i32 - c as i32;
        // Negate the total for the next step's calculation
        total = -total;
    }
    total == 0
}

fn main() {
    let mut input = String::new();
    // Read the entire line from stdin, handle errors as per original C's behavior (no error checking)
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    // Trim whitespace to mimic scanf's behavior of reading until whitespace
    let num_str = input.trim();
    // Determine result and print exactly as original C code
    println!("{}", if is_balanced(num_str) { "true" } else { "false" });
}