// Problem: Weekly Contest 437 Problem 1
use std::io::{self, BufRead};

/// Checks if the string has a special substring where a character appears exactly k times consecutively.
fn has_special_substring(s: &str, k: i32) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    let mut consecutive = 0;
    
    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || chars[i] != chars[i + 1] {
            if consecutive == k as usize {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse input: a string s and an integer k
    let input = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    let s = parts[0];
    let k: i32 = parts[1].parse().unwrap();
    
    // Call the function and print the result
    println!("{}", has_special_substring(s, k) as i32);
}