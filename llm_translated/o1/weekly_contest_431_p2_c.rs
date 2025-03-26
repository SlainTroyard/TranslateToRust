// LeetCode Weekly Contest 431 Problem 2 in Rust
// This program reads a single string from stdin, computes a score based on
// matching positions in separate stacks (mirroring the C code's logic), and
// prints the result to stdout.

use std::io::{self, BufRead};

/// Calculates the score by managing positions with 26 stacks (one per letter).
/// The logic matches the original C implementation exactly.
fn calculate_score(s: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut stacks = vec![Vec::new(); 26]; // Each element is a stack for a letter

    for (i, ch) in s.chars().enumerate() {
        // Convert character to index; 'a' -> 0, 'z' -> 25
        let c = (ch as u8 - b'a') as usize;
        // Check if the "matching" stack (25 - c) is not empty
        if !stacks[25 - c].is_empty() {
            // Calculate and add to score
            let top_pos = *stacks[25 - c].last().unwrap() as i64;
            ans += (i as i64) - top_pos;
            stacks[25 - c].pop();
        } else {
            // No match - push the current position onto the relevant stack
            stacks[c].push(i);
        }
    }

    ans
}

fn main() {
    // Read a single string from stdin (similar to scanf("%s", s) in C)
    let stdin = io::stdin();
    let mut input_string = String::new();
    // Read exactly one line (the C code effectively reads one whitespace-delimited string)
    stdin.lock().read_line(&mut input_string).unwrap();

    // Remove trailing newline
    let s = input_string.trim();
    
    let result = calculate_score(s);
    // Print result in the same format the C code uses (printf("%lld\n", result))
    println!("{}", result);
}