use std::io::{self, Write};

// Function to check if the string contains a substring of exactly `k` consecutive identical characters
fn has_special_substring(s: &str, k: usize) -> bool {
    let mut consecutive = 0;
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();

    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || chars[i] != chars[i + 1] {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into the string and the integer k
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return;
    }

    let s = parts[0];
    let k: usize = parts[1].parse().expect("k must be a valid integer");

    // Check if the string contains the special substring
    let result = has_special_substring(s, k);

    // Print the result as an integer (0 or 1)
    println!("{}", result as i32);
}