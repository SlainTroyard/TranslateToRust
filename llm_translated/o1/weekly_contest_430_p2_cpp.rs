use std::cmp;
use std::error::Error;
use std::io::{self, BufRead};

// A struct to encapsulate solution-related methods
struct Solution;

impl Solution {
    // Reproduces the same logic as the C++ answerString function
    fn answer_string(&self, s: &str, k: i32) -> String {
        // If k == 1, return the entire string as-is
        if k == 1 {
            return s.to_string();
        }

        let n = s.len();
        let mut ans = String::new();

        // Check all possible substrings as in the original C++ code
        for i in 0..n {
            let length = n - cmp::max((k - 1) as usize, i);
            let substring = &s[i..i + length];
            if substring > ans.as_str() {
                ans = substring.to_string();
            }
        }

        ans
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a solution instance
    let solution = Solution;

    // Read the string s from stdin (first input)
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // s is read as a single line input (similar to cin >> s in C++)
    let s = lines
        .next()
        .ok_or("No input found for s")??
        .trim()
        .to_string();

    // k is read as the next line (similar to cin >> k in C++)
    let k_line = lines
        .next()
        .ok_or("No input found for k")??;
    let k: i32 = k_line.trim().parse()?;

    // Compute the result
    let result = solution.answer_string(&s, k);

    // Output the result (same as cout << result << endl in C++)
    println!("{}", result);

    Ok(())
}