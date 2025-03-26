use std::io;

/// Checks if the sum of even-position digits equals the sum of odd-position digits in the input string.
///
/// # Arguments
///
/// * `num` - A string slice representing the number to check.
///
/// # Returns
///
/// * `bool` - True if sums are equal, false otherwise.
fn is_balanced(num: &str) -> bool {
    let (mut even_sum, mut odd_sum) = (0, 0);
    for (i, c) in num.chars().enumerate() {
        // Convert char to digit. Safe as per problem constraints (input is a valid number).
        let digit = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            even_sum += digit;
        } else {
            odd_sum += digit;
        }
    }
    even_sum == odd_sum
}

fn main() {
    let mut input = String::new();
    // Read input line, handling possible I/O errors with expect (as per problem constraints)
    io::stdin().read_line(&mut input).expect("Failed to read input");
    // Trim whitespace (including newline) and process the trimmed string
    let num = input.trim();
    let result = is_balanced(num);
    // Output exactly "true" or "false" as per original CPP code
    println!("{}", if result { "true" } else { "false" });
}