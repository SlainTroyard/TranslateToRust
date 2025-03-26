// LeetCode Weekly Contest 414 Problem 1 in Rust
// ---------------------------------------------
// This program replicates the exact logic and I/O format
// of the original C code, using idiomatic Rust where possible.

use std::io;

/// Replicates the behavior of the C 'atoi' function, taking
/// the initial numeric portion of the string and ignoring the rest.
fn parse_atoi(s: &str) -> i32 {
    let numeric_part: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    numeric_part.parse::<i32>().unwrap_or(0)
}

/// Builds the binary representation of a given integer from right to left,
/// storing the bits in 'buffer', starting at position '*ptr'.
fn fill_binary(mut num: i32, buffer: &mut [char], ptr: &mut usize) {
    // If the number is zero, store a single '0' bit
    if num == 0 {
        *ptr -= 1;
        buffer[*ptr] = '0';
    } else {
        // Otherwise, repeatedly extract bits and place them from right to left
        while num != 0 {
            let bit = (num & 1) as u8 + b'0';
            *ptr -= 1;
            buffer[*ptr] = bit as char;
            num >>= 1;
        }
    }
}

/// Converts a date string "YYYY-MM-DD" into a binary representation of
/// "year in binary-month in binary-day in binary", each separated by '-'.
/// This replicates the pointer-arithmetic logic from the original C code.
fn convert_date_to_binary(date: &str) -> String {
    // Extract day, month, and year by mimicking pointer offsets in C:
    let d = parse_atoi(date.get(8..).unwrap_or(""));
    let m = parse_atoi(date.get(5..).unwrap_or(""));
    let y = parse_atoi(date);

    // We'll build the final string from right to left into this array
    let mut buffer = ['\0'; 25];
    let mut ptr = 25; // Start just past the last valid index

    // Day
    fill_binary(d, &mut buffer, &mut ptr);
    ptr -= 1;
    buffer[ptr] = '-';

    // Month
    fill_binary(m, &mut buffer, &mut ptr);
    ptr -= 1;
    buffer[ptr] = '-';

    // Year
    fill_binary(y, &mut buffer, &mut ptr);

    // The final string is the substring of 'buffer' starting at 'ptr'
    buffer[ptr..].iter().collect()
}

fn main() {
    // Read input from stdin (same as scanf("%s", date) in C)
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read input");
    let date = date.trim();

    // Convert the date to binary representation
    let ans = convert_date_to_binary(date);

    // Print the result (same formatting as the C code)
    println!("{}", ans);
}