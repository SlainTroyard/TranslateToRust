// Problem: Weekly Contest 435 Problem 1

use std::io::{self, BufRead};

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    
    // Count occurrences of each letter
    for ch in s.bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    
    // Find maximum odd count and minimum even non-zero count
    let mut c1 = 0;  // Maximum odd count
    let mut c2 = 100;  // Minimum even non-zero count
    
    for i in 0..26 {
        if cnt[i] & 1 == 1 {  // Odd count
            c1 = c1.max(cnt[i]);
        } else if cnt[i] != 0 {  // Even non-zero count
            c2 = c2.min(cnt[i]);
        }
    }
    
    c1 - c2  // Return the difference
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read line");
    
    // Remove whitespace and newlines
    let s = input.trim();
    
    // Calculate and print the result
    println!("{}", max_difference(s));
}