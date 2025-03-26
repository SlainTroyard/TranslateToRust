use std::io::{self, BufRead};

/// A solution struct to replicate the C++ class interface
struct Solution;

impl Solution {
    /// Checks if the string `s` contains a substring where exactly `k` consecutive characters are the same.
    fn has_special_substring(&self, s: &str, k: usize) -> bool {
        let n = s.len();
        let mut cnt = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..n {
            cnt += 1;
            // If we've reached the end of the string OR the next character is different
            if i == n - 1 || chars[i] != chars[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    // We will gather tokens from standard input until we have at least
    // two tokens: one for the string `s` and one for the integer `k`.
    let stdin = io::stdin();
    let mut tokens: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?;
        tokens.extend(line.split_whitespace().map(String::from));
        if tokens.len() >= 2 {
            // Once we have at least two tokens, stop reading further
            break;
        }
    }

    // Ensure we have both `s` and `k` properly read
    if tokens.len() < 2 {
        // Mimic minimal error handling if not enough input is provided
        eprintln!("Not enough input data");
        return Ok(());
    }

    let s = &tokens[0];
    let k: usize = match tokens[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Failed to parse integer");
            return Ok(());
        }
    };

    // Create a solution instance to call the method
    let sol = Solution;
    let result = sol.has_special_substring(s, k);

    // Print 1 if true, otherwise 0, matching the C++ bool output
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}