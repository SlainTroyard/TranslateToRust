// Problem: Weekly Contest 437 Problem 1

use std::io;

fn has_special_substring(s: &str, k: usize) -> bool {
    let length = s.len();
    let mut consecutive = 0;
    let bytes = s.as_bytes();

    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || bytes[i] != bytes[i + 1] {
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

    // Split the input into the string and the integer
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return;
    }

    let s = parts[0];
    let k: usize = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid integer input");
            return;
        }
    };

    // Call the function and print the result
    let result = has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });
}