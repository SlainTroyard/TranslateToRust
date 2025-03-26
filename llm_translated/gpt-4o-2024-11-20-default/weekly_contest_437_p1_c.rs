use std::io::{self, Write};

fn has_special_substring(s: &str, k: usize) -> bool {
    let mut consecutive = 0;
    let bytes = s.as_bytes(); // Convert the string to bytes for easier indexing

    for i in 0..bytes.len() {
        consecutive += 1;

        // Check if the current character is the last in the series
        if i == bytes.len() - 1 || bytes[i] != bytes[i + 1] {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0; // Reset the counter
            }
        }
    }

    false
}

fn main() {
    let mut input = String::new();

    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Split into two parts: the string and the integer
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("Missing string input");
    let k: usize = parts
        .next()
        .expect("Missing integer input")
        .parse()
        .expect("Failed to parse integer");

    // Call the function and print the output
    let result = has_special_substring(s, k);
    println!("{}", result as i32); // Print boolean as 0 or 1
}