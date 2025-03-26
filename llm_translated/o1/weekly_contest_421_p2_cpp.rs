/// Translated from the original C++ LeetCode solution for:
/// Weekly Contest 421 Problem 2
///
/// This Rust program reads two tokens from stdin:
/// 1) A string `s`
/// 2) An integer `t`
/// Then it computes and prints the transformed string length according to the same
/// logic as the C++ code.

use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

/// A struct to replicate the C++ `Solution` class
struct Solution;

impl Solution {
    /// Replicates the C++ method `lengthAfterTransformations`.
    /// Takes a string `s` and an integer `t`, then applies the same transformations:
    /// - Count frequencies of each character from 'a' to 'z'
    /// - For each transformation:
    ///     - Shift counts for indices 0..24 to 1..25
    ///     - Add the count of 'z' to both indices 0 and 1
    ///     - Take results modulo MOD
    /// - Returns the sum of counts modulo MOD
    fn length_after_transformations(&self, s: &str, t: i32) -> i64 {
        let mut cnt = [0_i64; 26];

        // Count frequency of each character
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        // Perform t transformations
        for _ in 0..t {
            let mut nxt = [0_i64; 26];
            // Shift counts: for j in [0..24], put cnt[j] into nxt[j+1]
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            // The count of 'z' (cnt[25]) goes to both 0 and 1
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;

            // Update cnt
            cnt = nxt;
        }

        // Compute final answer
        let mut ans = 0_i64;
        for &val in &cnt {
            ans = (ans + val) % MOD;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Read the entire stdin into a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Tokenize input, matching the original "cin >> s >> t"
    let mut tokens = input.split_whitespace();

    // Extract the string `s`
    let s = tokens
        .next()
        .expect("Expected a string input for 's'")
        .to_string();

    // Extract integer `t`
    let t: i32 = tokens
        .next()
        .expect("Expected an integer input for 't'")
        .parse()
        .expect("Failed to parse 't' as an integer");

    // Create a solution instance and compute the final length
    let solution = Solution;
    let result = solution.length_after_transformations(&s, t);

    // Print the result (exactly like the original C++ `cout << result`)
    println!("{}", result);

    Ok(())
}