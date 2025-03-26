use std::io;

/// Converts a number to its binary string representation.
///
/// # Arguments
///
/// * `n` - The number to convert to binary.
///
/// # Returns
///
/// A `String` representing the binary form of `n`.
fn to_binary(mut n: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut s = String::new();
    while n > 0 {
        // Push the least significant bit as a character ('0' or '1')
        s.push(((n & 1) as u8 + b'0') as char);
        n >>= 1;
    }
    // Reverse to get the correct order (most significant bit first)
    s.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let date = input.trim();
    let parts: Vec<&str> = date.split('-').collect();
    
    // Parse each part into numbers, using expect for proper error handling
    let year = parts[0].parse::<u32>().expect("Invalid year");
    let month = parts[1].parse::<u32>().expect("Invalid month");
    let day = parts[2].parse::<u32>().expect("Invalid day");
    
    // Convert each component to binary
    let year_bin = to_binary(year);
    let month_bin = to_binary(month);
    let day_bin = to_binary(day);
    
    // Combine the binary strings with hyphens
    let result = format!("{}-{}-{}", year_bin, month_bin, day_bin);
    println!("{}", result);
}