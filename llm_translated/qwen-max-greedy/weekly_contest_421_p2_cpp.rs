use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input string and integer
    let s = lines.next().unwrap().expect("Failed to read input string");
    let t: usize = lines.next().unwrap().expect("Failed to read input integer").parse().expect("Failed to parse integer");

    // Create an instance of Solution and call the function
    let solution = Solution;
    let result = solution.length_after_transformations(s, t);

    // Print the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Computes the length after a series of transformations on the string.
    ///
    /// # Arguments
    /// * `s` - A string consisting of lowercase English letters.
    /// * `t` - The number of transformations to apply.
    ///
    /// # Returns
    /// The length of the string after `t` transformations.
    pub fn length_after_transformations(s: String, t: usize) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt = [0; 26];

        // Count the frequency of each character in the string
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        // Apply the transformations
        for _ in 0..t {
            let mut nxt = [0; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            cnt = nxt;
        }

        // Calculate the final length
        let mut ans: i64 = 0;
        for &count in &cnt {
            ans = (ans + count) % MOD;
        }

        ans
    }
}