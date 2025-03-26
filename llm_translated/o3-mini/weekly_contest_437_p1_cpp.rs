use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a buffered reader for standard input
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all lines from standard input and split them by whitespace,
    // preserving the same input parsing as the original C++ code.
    for line in stdin.lock().lines() {
        let line = line?;
        tokens.extend(line.split_whitespace().map(String::from));
    }

    // Ensure that we have at least two tokens: one for the string and one for the integer k.
    if tokens.len() < 2 {
        return Err("Expected at least 2 tokens (a string and an integer)".into());
    }

    // The first token is our string, the second token is the integer k.
    let s = tokens[0].clone();
    let k: usize = tokens[1].parse()?;

    // Call the solution function
    let result = Solution::has_special_substring(&s, k);

    // Print the result.
    // The original C++ code outputs the boolean result as "1" for true and "0" for false.
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}

struct Solution;

impl Solution {
    // This function implements the same logic as the C++ hasSpecialSubstring method.
    fn has_special_substring(s: &str, k: usize) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut cnt = 0;

        for i in 0..n {
            cnt += 1;
            // If we're at the last character or the current character is different from the next,
            // check if the count equals k.
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