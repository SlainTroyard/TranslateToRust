// Problem: Weekly Contest 420 Problem 1
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();

        for c in target.chars() {
            s.push('a'); // Placeholder character
            for j in 'a'..=c {
                s.pop(); // Remove the last character
                s.push(j); // Replace it with the current character
                ans.push(s.clone());
            }
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target = input.trim().to_string();

    // Create a Solution instance and call the method
    let solution = Solution;
    let ans = solution.string_sequence(target);

    // Write output to stdout
    let mut stdout = io::stdout();
    let mut handle = stdout.lock();
    for s in ans {
        write!(handle, "{} ", s).expect("Failed to write output");
    }
    writeln!(handle).expect("Failed to write newline");
}