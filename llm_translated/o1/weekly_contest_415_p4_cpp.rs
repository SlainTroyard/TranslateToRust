// Weekly Contest 415 Problem 4
// Translated from C++ to Rust

use std::collections::HashSet;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

// We will use the 'rand' crate for random number generation.
// In Cargo.toml, include:
// [dependencies]
// rand = "0.8"
use rand::prelude::*;

/// A struct that encapsulates the logic for the solution.
struct Solution {
    base: i64,
    // We don't store MOD here since it's a constant, but we could if desired.
}

impl Solution {
    /// Creates a new Solution, picking a random base in [8e8 ..= 9e8].
    fn new() -> Self {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(_) => 0,
        };
        let mut rng = StdRng::seed_from_u64(seed);
        let base = rng.gen_range(800_000_000..=900_000_000) as i64;
        Solution { base }
    }

    /// Returns the minimum number of valid strings needed, or -1 if impossible.
    fn min_valid_strings(&self, words: &[String], target: &str) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }

        // Prepare for rolling hash
        const MOD: i64 = 1_070_777_777;
        let mut pow_base = vec![0i64; n + 1];
        let mut pre_hash = vec![0i64; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * self.base) % MOD;
            pre_hash[i + 1] = (pre_hash[i] * self.base + target.as_bytes()[i] as i64) % MOD;
        }

        // Function to get the hash of the substring target[l..r]
        let sub_hash = |l: usize, r: usize| {
            let raw = pre_hash[r]
                .wrapping_sub((pre_hash[l] * pow_base[r - l]) % MOD)
                .rem_euclid(MOD);
            raw
        };

        // Find the maximum word length among all words
        let mut max_len = 0;
        for w in words {
            if w.len() > max_len {
                max_len = w.len();
            }
        }

        // Build hash sets for all prefixes of all words of each possible length.
        // sets[k] will contain hashes for all words' prefixes of length k+1.
        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        for w in words {
            let mut h = 0i64;
            for (j, &b) in w.as_bytes().iter().enumerate() {
                h = ((h * self.base) + b as i64) % MOD;
                sets[j].insert(h);
            }
        }

        // Computes the maximum possible jump (prefix match) starting at index i in target.
        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            let mut right = std::cmp::min(n - i, max_len) + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                let h = sub_hash(i, i + mid);
                if sets[mid - 1].contains(&h) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            left
        };

        // Greedy approach to cover target with minimum segments
        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;

        for i in 0..n {
            nxt_r = nxt_r.max(i + max_jump(i));
            if i == cur_r {
                if i == nxt_r {
                    return -1; // Impossible to move further
                }
                cur_r = nxt_r;
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of words
    let words_size: usize = lines
        .next()
        .expect("Expected input line for words_size")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse words_size");

    // Read each word's length and the word itself
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let line = lines
            .next()
            .expect("Expected input line for word length and word")
            .expect("Failed to read line");
        let mut parts = line.split_whitespace();
        let word_len: usize = parts
            .next()
            .expect("Failed to read word length")
            .parse()
            .expect("Failed to parse word length");
        let w = parts
            .next()
            .expect("Failed to read word string")
            .to_string();
        // The C++ code does not validate word length; we'll trust the input.
        assert_eq!(word_len, w.len(), "Word length does not match actual string length");
        words.push(w);
    }

    // Read target length and target
    let line = lines
        .next()
        .expect("Expected input line for target length and target")
        .expect("Failed to read line");
    let mut parts = line.split_whitespace();
    let target_len: usize = parts
        .next()
        .expect("Failed to read target length")
        .parse()
        .expect("Failed to parse target length");
    let target = parts
        .next()
        .expect("Failed to read target string")
        .to_string();

    // (Optional) verify length
    assert_eq!(target_len, target.len(), "Target length does not match actual string length");

    // Solve
    let sol = Solution::new();
    let ans = sol.min_valid_strings(&words, &target);

    // Output answer
    println!("{}", ans);
}