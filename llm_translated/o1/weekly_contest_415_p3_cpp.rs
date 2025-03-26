// Translated Rust code for "Weekly Contest 415 Problem 3"
// Requirements:
// 1. Translates the ENTIRE file as a complete program, preserving logic exactly
// 2. Uses idiomatic Rust with proper error handling
// 3. Maintains the EXACT SAME stdin/stdout format as in the original C++ code
// 4. Comments added for clarity

use std::collections::HashSet;
use std::io::{self, BufRead};
use std::time::SystemTime;

// We'll need a random base similar to the original C++ code, so we use the "rand" crate
// In a standalone environment, ensure "rand" is available.
use rand::{distributions::Uniform, prelude::*};

/// A helper function to read a single line from stdin.
fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

/// Our equivalent of the C++ Solution class
struct Solution;

impl Solution {
    /// Translated method from C++: int minValidStrings(vector<string>& words, string target)
    fn min_valid_strings(&self, words: &[String], target: &str) -> i32 {
        // n = length of target
        let n = target.len();
        // MOD from the C++ code
        const MOD: i64 = 1_070_777_777;

        // We replicate the C++ random generator for BASE in [8e8..9e8].
        let seed = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let dist = Uniform::new_inclusive(800_000_000i64, 900_000_000i64);
        let base = dist.sample(&mut rng);

        // Precompute powers of base and prefix hashes for the target string
        let mut pow_base = vec![0i64; n + 1];
        let mut pre_hash = vec![0i64; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = ((pow_base[i] as i128 * base as i128) % MOD as i128) as i64;
            let c_val = target.as_bytes()[i] as i64;
            let tmp = ((pre_hash[i] as i128 * base as i128) + (c_val as i128)) % (MOD as i128);
            pre_hash[i + 1] = tmp as i64;
        }

        // sub_hash(l, r) computes the rolling hash for target[l..r]
        // exactly the same logic as the C++ lambda
        let sub_hash = |l: usize, r: usize| -> i64 {
            let left_part = pre_hash[l] as i128 * pow_base[r - l] as i128;
            let diff = (pre_hash[r] as i128 - (left_part % MOD as i128)) % MOD as i128;
            ((diff + MOD as i128) % MOD as i128) as i64
        };

        // Determine the maximum length among the words
        let mut max_len = 0usize;
        for w in words {
            if w.len() > max_len {
                max_len = w.len();
            }
        }

        // Build a vector of sets. For each j, we store the hashes of length j+1 for all words.
        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        for w in words {
            let mut h: i64 = 0;
            for (j, ch) in w.chars().enumerate() {
                let c_val = ch as u8 as i64;
                h = ((h as i128 * base as i128) + c_val as i128) % MOD as i128 as i64;
                if j < max_len {
                    sets[j].insert(h);
                }
            }
        }

        // A function that uses binary search to find how far we can jump from index i
        // in the target string with some word in "words".
        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            // We can jump up to min(n - i, max_len) in length
            let mut right = std::cmp::min(n - i, max_len) + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                let hash_val = sub_hash(i, i + mid);
                if sets[mid - 1].contains(&hash_val) {
                    left = mid; // we can jump mid
                } else {
                    right = mid; // we cannot jump mid
                }
            }
            left
        };

        // Greedy approach to cover the entire target with a minimal number of substrings
        let mut ans = 0;
        let mut cur_r = 0; // current rightmost position covered
        let mut nxt_r = 0; // the best extension we can achieve

        for i in 0..n {
            nxt_r = std::cmp::max(nxt_r, i + max_jump(i));
            // If we have reached the current boundary, we need to add 1 to the count
            if i == cur_r {
                // If we cannot extend any further, it's impossible
                if i == nxt_r {
                    return -1;
                }
                cur_r = nxt_r;
                ans += 1;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Read the number of words
    let words_size_line = read_line()?;
    let words_size: usize = words_size_line.trim().parse().unwrap();

    // Read each word (preceded by its length, which is read but not directly used)
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let line = read_line()?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        // parts[0] is word_len, parts[1] is the actual string
        let _word_len: usize = parts[0].parse().unwrap(); // read but not used
        let w = parts[1].to_string();
        words.push(w);
    }

    // Read the target length and target
    let line = read_line()?;
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    let _target_len: usize = parts[0].parse().unwrap(); // read but not used
    let target = parts[1].to_string();

    // Create a Solution struct and compute the result
    let sol = Solution;
    let result = sol.min_valid_strings(&words, &target);

    // Output the result exactly as in the C++ code
    println!("{}", result);

    Ok(())
}