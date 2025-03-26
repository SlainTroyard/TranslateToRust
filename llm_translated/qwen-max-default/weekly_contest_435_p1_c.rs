// Problem: Weekly Contest 435 Problem 1
use std::io;

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    for c in s.chars() {
        if let Some(index) = c.to_ascii_lowercase().to_digit(36) {
            cnt[index as usize - 10] += 1;
        }
    }

    let mut c1 = 0;
    let mut c2 = 100;
    for &count in &cnt {
        if count % 2 == 1 {
            c1 = c1.max(count);
        } else if count != 0 {
            c2 = c2.min(count);
        }
    }

    c1 - c2
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string(); // Trim the newline and any trailing whitespace

    // Call the function and print the result
    let result = max_difference(&input);
    println!("{}", result);
}