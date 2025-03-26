use std::io::{self, Read};

/// A structure to hold our solution logic (mirroring the C++ class).
struct Solution;

impl Solution {
    /// Translated from the C++ method:
    /// Counts the number of substrings in `s` following the original logic.
    fn number_of_substrings(&self, s: &str, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0usize;
        let mut cnt = [0; 26]; // Array to store character counts

        // Iterate over each character in the string
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;

            // While the count of this character is at least k,
            // shrink the window from the left
            while cnt[idx] >= k {
                let left_idx = (s.as_bytes()[left] - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }

            // For each new character, increase the answer by 'left'
            ans += left as i32;
        }

        ans
    }
}

fn main() -> io::Result<()> {
    // Read the entire input from stdin.
    // The original C++ code uses "cin >> s >> k;", which ignores whitespace
    // and takes the next two tokens as s and k, respectively.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace and parse the first two tokens
    let mut tokens = input.split_whitespace();
    let s = tokens.next().expect("Failed to read string 's'").to_string();
    let k_str = tokens.next().expect("Failed to read integer 'k'");
    let k: i32 = k_str.parse().expect("Failed to parse integer 'k'");

    // Create a Solution instance and compute the answer
    let sol = Solution;
    let result = sol.number_of_substrings(&s, k);

    // Print the result (matching the original C++ "cout << ... << endl;")
    println!("{}", result);

    Ok(())
}