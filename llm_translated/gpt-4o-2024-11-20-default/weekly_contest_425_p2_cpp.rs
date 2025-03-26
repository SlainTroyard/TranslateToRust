use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Function to check if it's possible to rearrange strings `s` and `t` into equal chunks
    pub fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
        let n = s.len(); // Total length of the strings
        let chunk_size = n / k; // Calculate chunk size
        let mut mp: HashMap<String, i32> = HashMap::new();

        // Count occurrences of chunks in string `s`
        for i in (0..n).step_by(chunk_size) {
            let chunk = &s[i..i + chunk_size];
            *mp.entry(chunk.to_string()).or_insert(0) += 1;
        }

        // Subtract occurrences of chunks from string `t`
        for i in (0..n).step_by(chunk_size) {
            let chunk = &t[i..i + chunk_size];
            *mp.entry(chunk.to_string()).or_insert(0) -= 1;
        }

        // If all chunk counts are zero, strings are rearrangeable
        mp.values().all(|&count| count == 0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input values
    let s = lines.next().ok_or("Missing input for `s`")??;
    let t = lines.next().ok_or("Missing input for `t`")??;
    let k: usize = lines
        .next()
        .ok_or("Missing input for `k`")??
        .trim()
        .parse()
        .map_err(|_| "Failed to parse `k` as usize")?;

    // Create solution instance and call the function
    let solution = Solution;
    let result = solution.is_possible_to_rearrange(&s, &t, k);

    // Output the result, matching the original format
    if result {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}