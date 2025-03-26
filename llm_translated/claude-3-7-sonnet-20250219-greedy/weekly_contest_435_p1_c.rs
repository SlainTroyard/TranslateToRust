// Problem: Weekly Contest 435 Problem 1
use std::io::{self, BufRead};

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    
    // Count occurrences of each character
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    
    // Initialize c1 to 0 (max odd count) and c2 to 100 (min even non-zero count)
    let mut c1 = 0;
    let mut c2 = 100;
    
    for i in 0..26 {
        if cnt[i] & 1 == 1 {
            // If count is odd, update c1 if this count is larger
            c1 = c1.max(cnt[i]);
        } else if cnt[i] != 0 {
            // If count is even and non-zero, update c2 if this count is smaller
            c2 = c2.min(cnt[i]);
        }
    }
    
    // Return the difference
    c1 - c2
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string
    if let Some(Ok(s)) = lines.next() {
        // Calculate and print the result
        println!("{}", max_difference(&s));
    }
}