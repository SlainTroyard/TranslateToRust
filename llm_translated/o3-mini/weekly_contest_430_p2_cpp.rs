use std::cmp;
use std::error::Error;
use std::io::{self, Read};

/// Returns the lexicographically largest substring based on the problem requirement.
///
/// If k equals 1, just return the original string. Otherwise, iterate
/// over all possible starting indices and choose the maximum lexicographic substring.
///
/// # Arguments
///
/// * `s` - The input string slice.
/// * `k` - Parameter k determining the substring length constraints.
///
/// # Returns
///
/// A String which is the answer according to the defined algorithm.
fn answer_string(s: &str, k: usize) -> String {
    // If k is 1, return s immediately.
    if k == 1 {
        return s.to_string();
    }
    let n = s.len();
    let mut ans = String::new();
    // Iterate over each possible starting index.
    // Since the original C++ code uses s.substr(i, n - max(k - 1, i)),
    // we compute the substring starting at i with length = n - max(k - 1, i).
    for i in 0..n {
        // Calculate the length of the candidate substring.
        let len = n - cmp::max(k - 1, i);
        // Using byte indices directly is safe here because we assume the input is ASCII.
        let candidate = &s[i..i + len];
        // Update ans if the current candidate is lexicographically larger.
        if candidate > ans.as_str() {
            ans = candidate.to_string();
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input by whitespace to simulate the behavior of C++'s `cin >> s; cin >> k;`
    let mut tokens = input.split_whitespace();
    
    // The first token corresponds to the input string.
    let s = tokens.next().ok_or("Missing input string")?;
    
    // The second token corresponds to the integer k.
    let k_token = tokens.next().ok_or("Missing input integer k")?;
    let k = k_token.parse::<usize>()?;
    
    // Call the answer_string function.
    let result = answer_string(s, k);
    
    // Print the result (output exactly as in the original code).
    println!("{}", result);
    
    Ok(())
}