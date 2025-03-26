// Problem: Weekly Contest 435 Problem 2

use std::io::{self, BufRead};

/// A simple struct to encapsulate the solution's method
struct Solution;

impl Solution {
    /// Replicates the logic of the C++ maxDistance function.
    /// Given a string of directions 'N', 'S', 'E', 'W' and an integer k,
    /// returns the maximum possible distance adhering to the original algorithm.
    fn max_distance(&self, s: &str, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        
        // Iterate over each index and character in the directions string
        for (i, ch) in s.chars().enumerate() {
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                _ => x -= 1,
            }

            // Translate: ans = max(ans, min(abs(x) + abs(y) + k * 2, i + 1))
            let distance = (x.abs() + y.abs() + k * 2)
                .min((i + 1) as i32);
            ans = ans.max(distance);
        }
        
        ans
    }
}

/// Reads two tokens (a string and an integer) from standard input,
/// mirroring the behavior of "cin >> s >> k" in C++ by ignoring all
/// whitespace (across lines if needed).
fn read_two_tokens() -> io::Result<(String, i32)> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Collect tokens from stdin until we have two
    for line in stdin.lock().lines() {
        let line = line?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
            if tokens.len() == 2 {
                let s = tokens[0].clone();
                let k: i32 = tokens[1].parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Failed to parse integer for k")
                })?;
                return Ok((s, k));
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough input tokens"))
}

fn main() -> io::Result<()> {
    // Read the two tokens from stdin
    let (s, k) = read_two_tokens()?;

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.max_distance(&s, k);

    // Output the result to stdout
    println!("{}", result);

    Ok(())
}