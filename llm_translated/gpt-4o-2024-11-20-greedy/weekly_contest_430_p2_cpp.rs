use std::cmp::max;
use std::io::{self, Write};

struct Solution;

impl Solution {
    fn answer_string(&self, s: String, k: usize) -> String {
        if k == 1 {
            return s;
        }
        let n = s.len();
        let mut ans = String::new();
        for i in 0..n {
            // Calculate the substring length based on the logic in the original code
            let substr_len = n - max(k - 1, i);
            let current_substr = &s[i..i + substr_len];
            ans = max(ans, current_substr.to_string());
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read the input string `s`
    stdin.read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();

    // Read the integer `k`
    input.clear();
    stdin.read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Failed to parse integer");

    // Create a Solution instance and call the answer_string method
    let solution = Solution;
    let result = solution.answer_string(s, k);

    // Print the result
    writeln!(stdout_lock, "{}", result).expect("Failed to write output");
}