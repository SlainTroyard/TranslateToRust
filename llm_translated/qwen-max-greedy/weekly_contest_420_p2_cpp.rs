use std::io::{self, BufRead};

/// Represents the solution for the problem.
struct Solution;

impl Solution {
    /// Calculates the number of substrings with at most `k` repeating characters.
    fn number_of_substrings(s: &str, k: usize) -> usize {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];

        for (right, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            cnt[index] += 1;

            while cnt[index] > k {
                cnt[(s.chars().nth(left).unwrap() as u8 - b'a') as usize] -= 1;
                left += 1;
            }

            ans += left;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let s = parts[0];
            let k: usize = parts[1].parse().expect("Failed to parse k");

            // Create an instance of Solution and call the method
            let sol = Solution;
            let result = sol.number_of_substrings(s, k);

            // Print the result to stdout
            println!("{}", result);
        } else {
            eprintln!("Invalid input format");
        }
    } else {
        eprintln!("No input provided");
    }
}