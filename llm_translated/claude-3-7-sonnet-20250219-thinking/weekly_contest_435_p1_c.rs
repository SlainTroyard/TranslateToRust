// Problem: Weekly Contest 435 Problem 1
use std::io;

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    let mut c1 = 0;
    let mut c2 = 100;
    
    // Count occurrences of each character
    for ch in s.chars() {
        cnt[(ch as usize) - ('a' as usize)] += 1;
    }
    
    // Find maximum odd count and minimum even non-zero count
    for i in 0..26 {
        if cnt[i] & 1 == 1 {  // If count is odd
            c1 = if cnt[i] > c1 { cnt[i] } else { c1 };
        } else if cnt[i] != 0 {  // If count is even and not zero
            c2 = if cnt[i] < c2 { cnt[i] } else { c2 };
        }
    }
    
    c1 - c2
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");
    
    // Trim whitespace and newlines
    let s = s.trim();
    
    // Calculate and print the result
    println!("{}", max_difference(s));
}