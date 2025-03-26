use std::error::Error;
use std::io::{self, Read};

/// This function generates a sequence of strings based on the given target.
/// For each character in target, it appends a placeholder 'a' to the current string,
/// then replaces the last character with every character from 'a' up to the target character,
/// adding each new string to the answer vector.
/// This logic mimics the C++ solution from LeetCode Weekly Contest 420 Problem 1.
fn string_sequence(target: &str) -> Vec<String> {
    let mut ans = Vec::new();
    let mut s = String::new();
    for c in target.chars() {
        // Append a placeholder to 's'
        s.push('a');
        
        // Iterate from 'a' to the current target character `c`.
        // Since these are ASCII characters, we can safely use char ranges.
        for j in 'a'..=c {
            // Replace the last character in 's' by removing it and then pushing j.
            // This emulates "s.back() = j" from the C++ code.
            s.pop();
            s.push(j);
            ans.push(s.clone());
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin.
    // The original C++ code uses "cin >> target" which extracts the first token,
    // so we mimic that by taking the first whitespace-separated token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Use split_whitespace to parse the first token as the target string.
    // If no token is found, we treat target as an empty string.
    let target = input.split_whitespace().next().unwrap_or("");
    
    // Generate the sequence based on the target.
    let ans = string_sequence(target);
    
    // Output the sequence with each string separated by a space,
    // and print a newline at the end to match the original C++ format.
    for s in ans {
        print!("{} ", s);
    }
    println!();
    
    Ok(())
}