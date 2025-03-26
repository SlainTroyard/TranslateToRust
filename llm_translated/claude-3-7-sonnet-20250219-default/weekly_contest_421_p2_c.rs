// Translated from C to Rust: LeetCode Weekly Contest 421 Problem 2
use std::io::{self, BufRead};

/// Calculates the length of a string after performing a series of transformations.
/// 
/// # Arguments
/// * `s` - The input string
/// * `t` - The number of transformations to perform
///
/// # Returns
/// The length of the transformed string, modulo 10^9 + 7
fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst = [0; 26];
    let modulo = 1000000007;
    
    // Count occurrences of each letter in the input string
    for c in s.bytes() {
        lst[(c - b'a') as usize] += 1;
    }
    
    // Perform transformations
    let mut t_remaining = t;
    while t_remaining > 0 {
        // Save the count of 'z'
        let z = lst[25];
        
        // Shift all letters (except 'a') to the next letter
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        
        // Apply transformation rules for 'a' and 'b'
        lst[1] = (lst[0] + z) % modulo; // 'b' gets 'a' + 'z'
        lst[0] = z;                     // 'a' becomes 'z'
        
        t_remaining -= 1;
    }
    
    // Calculate the final length by summing all letter counts
    let mut ans = 0;
    for i in 0..26 {
        ans = (ans + lst[i]) % modulo;
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the input string
    let s = lines.next().unwrap().unwrap();
    
    // Read the number of transformations
    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Calculate and print the result
    print!("{}", length_after_transformations(&s, t));
}