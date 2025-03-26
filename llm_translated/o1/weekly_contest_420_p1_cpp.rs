// Weekly Contest 420 Problem 1
//
// This Rust program is an idiomatic translation of the provided C++ code.
// It reads one token from stdin (mimicking "cin >> target"), then for each
// character in 'target', it generates a series of strings, printing them
// separated by a space, and finally a newline.

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // This method replicates the logic of the original 'stringSequence' function.
    // For each character in 'target', append a placeholder 'a' to 's', then
    // replace the last character of 's' from 'a' up to that character (inclusive),
    // pushing each intermediate string into 'ans'.
    fn string_sequence(&self, target: &str) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        for c in target.chars() {
            s.push('a'); // placeholder
            for j in 'a'..=c {
                // Replace the last character of s with j
                let last_idx = s.len() - 1;
                s.replace_range(last_idx..last_idx+1, &j.to_string());
                ans.push(s.clone());
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Read a single token from stdin, similar to 'cin >> target' in C++.
    // If multiple tokens exist on one line, only the first one is taken.
    // If the line is empty, we default to an empty string.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut target = String::new();
    while let Some(Ok(line)) = lines.next() {
        if !line.trim().is_empty() {
            target = line.split_whitespace().next().unwrap_or("").to_string();
            break;
        }
    }

    // Create the solution object and compute the result
    let solution = Solution;
    let ans = solution.string_sequence(&target);

    // Print the results exactly as in the original C++ code: each string
    // separated by a space, then a final newline.
    for s in ans {
        print!("{} ", s);
    }
    println!();

    Ok(())
}