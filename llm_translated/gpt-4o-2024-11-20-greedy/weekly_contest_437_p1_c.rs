use std::io::{self, Write};

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
    // Create a buffer for stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split the input into the string `s` and the integer `k`
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().expect("Expected a string input");
    let k: usize = parts
        .next()
        .expect("Expected an integer input")
        .parse()
        .expect("Failed to parse integer");

    // Call the function and print the result
    let result = has_special_substring(s, k);
    println!("{}", result as i32); // Output 1 for true, 0 for false
}