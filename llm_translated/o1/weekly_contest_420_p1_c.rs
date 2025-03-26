// Problem: Weekly Contest 420 Problem 1
// Translated from C to Rust, preserving logic and I/O format

use std::io::{self, BufRead};
use std::error::Error;

/// Translates the C function `stringSequence` into Rust.
/// For each character in `target`, this function generates all strings
/// formed by the substring of `target` up to (but not including) that character,
/// plus every character from 'a' to the character in `target`.
fn string_sequence(target: &str) -> Vec<String> {
    // Calculate total number of strings that will be generated
    let return_size = target.bytes().map(|b| (b as i32) - 96).sum::<i32>();

    // Reserve space to store all resulting strings
    let mut ans = Vec::with_capacity(return_size as usize);

    // `l` is an index tracking how far into the original target we are
    // For each character in the target, generate all strings from 'a' to that character
    for (l, &tbyte) in target.as_bytes().iter().enumerate() {
        let ch = tbyte as char;
        for c in 'a'..=ch {
            // Compose the new string by taking the first `l` chars of `target`
            // and appending the character `c`
            let mut new_str = String::with_capacity(l + 1);
            new_str.push_str(&target[..l]);
            new_str.push(c);
            ans.push(new_str);
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read one token (like `scanf("%s", ...)`); ignore anything after whitespace
    let stdin = io::stdin();
    let mut line = String::new();
    // Read one line from input
    stdin.lock().read_line(&mut line)?;
    // Extract the first token in the line (if any)
    let target = line.split_whitespace().next().unwrap_or("");

    // Compute the sequence of strings
    let ans = string_sequence(target);

    // Print each string followed by a space, matching original C output
    for s in &ans {
        print!("{} ", s);
    }
    println!(); // Print final newline

    Ok(())
}