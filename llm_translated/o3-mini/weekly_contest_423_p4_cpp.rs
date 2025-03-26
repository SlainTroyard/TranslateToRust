use std::io::{self, BufRead};

const MOD: i32 = 1_000_000_007;

// Struct to hold the DP cache, precomputed cnt array and the target value k.
struct Solution {
    // 3D DP table: dp[position][tight_flag as usize][set_bits] where:
    // - position: current index in the binary string s (0-indexed)
    // - tight_flag: 1 if the digits chosen so far are exactly the same as s's prefix; 0 otherwise.
    // - set_bits: the number of '1's chosen so far.
    dp: Vec<Vec<Vec<Option<i32>>>>,
    // Precomputed cnt array storing the "chain length" for numbers 0..=800.
    cnt: Vec<i32>,
    // Target k value.
    k: i32,
}

impl Solution {
    // The recursive DP function.
    // s_chars: slice of characters from the string s.
    // i: current position in the string.
    // tight: indicates if the current prefix is 'tight' (exact match to s's prefix).
    // set_bits: the count of '1's chosen so far.
    fn solve(&mut self, s_chars: &[char], i: usize, tight: bool, set_bits: usize) -> i32 {
        // Base condition: when we've processed all bits.
        if i == s_chars.len() {
            // If we are still "tight" or haven't chosen any 1's (set_bits == 0),
            // by the original logic, return 0.
            // Otherwise, if the chain length for set_bits is less than k, return 1; else 0.
            if tight || set_bits == 0 {
                return 0;
            } else {
                return if self.cnt[set_bits] < self.k { 1 } else { 0 };
            }
        }
        
        // Convert the 'tight' boolean into index 0/1 for the DP table.
        let tight_idx = if tight { 1 } else { 0 };

        // Return cached result if available.
        if let Some(cached) = self.dp[i][tight_idx][set_bits] {
            return cached;
        }
        
        let mut res = 0;
        
        if tight {
            // When we are tight, we must follow the digit in s.
            if s_chars[i] == '0' {
                // Only one option: choose 0 and remain tight.
                res = self.solve(s_chars, i + 1, true, set_bits);
            } else {
                // When the current digit is '1', we have two options:
                // 1) Choose 1: remain tight and increment set_bits.
                res = self.solve(s_chars, i + 1, true, set_bits + 1);
                // 2) Choose 0: break the tightness.
                res = (res + self.solve(s_chars, i + 1, false, set_bits)) % MOD;
            }
        } else {
            // When not tight, we have two free choices at each position:
            // 1) Choose 1 (increment set_bits).
            res = self.solve(s_chars, i + 1, false, set_bits + 1);
            // 2) Choose 0.
            res = (res + self.solve(s_chars, i + 1, false, set_bits)) % MOD;
        }
        
        // Cache and return the computed result.
        self.dp[i][tight_idx][set_bits] = Some(res);
        res
    }

    // Main function to count k-reducible numbers.
    // Takes the string s (in binary) and integer k as input.
    fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        
        // Precompute the cnt array for numbers 0..=800.
        // cnt[i] = 1 + cnt[popcount(i)] for i >= 2.
        let mut cnt = vec![0; 801];
        for i in 2..=800 {
            let pop = (i as i32).count_ones() as usize;
            cnt[i] = 1 + cnt[pop];
        }
        
        // Initialize the DP table.
        // Dimensions: (n+1) x 2 x (n+1)
        // Since set_bits can be at most n (the length of s), we use n+1.
        let dp = vec![vec![vec![None; n + 1]; 2]; n + 1];
        
        // Create a Solution instance.
        let mut sol = Solution { dp, cnt, k };
        // Start from the beginning: index 0, tight state true, and 0 set bits.
        sol.solve(&s_chars, 0, true, 0)
    }
}

fn main() -> io::Result<()> {
    // Use standard input handling
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read all tokens from standard input.
    // The original C++ code uses "cin >> s >> k",
    // which supports inputs on the same line or separate lines.
    let mut inputs: Vec<String> = Vec::new();
    while let Some(line_result) = iterator.next() {
        let line = line_result?;
        if !line.trim().is_empty() {
            for token in line.trim().split_whitespace() {
                inputs.push(token.to_string());
            }
        }
    }

    // Ensure we have at least two tokens: a string and an integer.
    if inputs.len() < 2 {
        eprintln!("Invalid input. Expected at least a string and an integer.");
        return Ok(());
    }

    // The first token is the binary string, and the second is integer k.
    let s = inputs[0].clone();
    let k: i32 = inputs[1].parse().expect("Invalid integer for k.");

    // Get the result using the solution function.
    let result = Solution::count_k_reducible_numbers(s, k);
    // Output the result exactly as in the original C++ code.
    println!("{}", result);
    Ok(())
}