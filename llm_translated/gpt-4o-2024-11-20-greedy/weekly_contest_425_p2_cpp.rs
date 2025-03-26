use std::collections::HashMap;
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
        let n = s.len();
        let size = n / k;
        let mut mp: HashMap<&str, i32> = HashMap::new();

        // Count substrings in `s`
        for i in (0..n).step_by(size) {
            let substring = &s[i..i + size];
            *mp.entry(substring).or_insert(0) += 1;
        }

        // Subtract counts for substrings in `t`
        for i in (0..n).step_by(size) {
            let substring = &t[i..i + size];
            *mp.entry(substring).or_insert(0) -= 1;
        }

        // Check if all counts are zero
        mp.values().all(|&value| value == 0)
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Read input strings `s` and `t`, and integer `k`
    stdin.read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();
    input.clear();

    stdin.read_line(&mut input).expect("Failed to read input");
    let t = input.trim().to_string();
    input.clear();

    stdin.read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Failed to parse integer");

    // Call the solution function
    let solution = Solution;
    let result = solution.is_possible_to_rearrange(&s, &t, k);

    // Print the result
    if result {
        writeln!(handle, "true").expect("Failed to write output");
    } else {
        writeln!(handle, "false").expect("Failed to write output");
    }
}