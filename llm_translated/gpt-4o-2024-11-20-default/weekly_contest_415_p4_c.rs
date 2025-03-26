use std::collections::HashSet;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;

const MOD: u64 = 1_070_777_777;

// Helper function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u64 {
    let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut seed = epoch_time ^ 0x12345678; // Randomization based on epoch time
    seed = (seed * 6364136223846793005).wrapping_add(1); // Basic pseudo-random multiplier
    800_000_000 + (seed % 100_000_000)
}

// Function to compute the minimum number of valid strings
fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    // Step 1: Determine the length of the target string
    let n = target.len();

    // Step 2: Find the maximum length among all words
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    // Step 3: Initialize the base for hashing
    let base = get_random_base();

    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base = vec![1u64; n + 1];
    for i in 1..=n {
        pow_base[i] = (pow_base[i - 1] * base) % MOD;
    }

    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash = vec![0u64; n + 1];
    for (i, &c) in target.as_bytes().iter().enumerate() {
        pre_hash[i + 1] = (pre_hash[i] * base + c as u64) % MOD;
    }

    // Step 6: Initialize string prefix hash sets for each possible prefix length
    let mut sets: Vec<HashSet<u64>> = vec![HashSet::new(); max_len];

    // Step 7: Populate sets with prefix hashes from the words
    for word in words {
        let mut h = 0u64;
        for (j, &c) in word.as_bytes().iter().enumerate().take(max_len) {
            h = (h * base + c as u64) % MOD;
            sets[j].insert(h);
        }
    }

    // Step 8: Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..nxt_r+1)
            let current_hash = if nxt_r + 1 > n {
                break;
            } else {
                (MOD + pre_hash[nxt_r + 1]
                    - pre_hash[i] * pow_base[nxt_r + 1 - i] % MOD)
                    % MOD
            };

            // Check if the current hash exists in the corresponding hash set
            let prefix_len = nxt_r - i;
            if prefix_len < max_len && sets[prefix_len].contains(&current_hash) {
                nxt_r += 1;
            } else {
                break;
            }
        }

        // If we've reached the end of the current cover range
        if i == cur_r {
            if i == nxt_r {
                return -1;
            }
            cur_r = nxt_r;
            ans += 1;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input: number of words
    let words_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("Invalid number format");

    // Input: words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }

    // Input: target length and target string
    let target_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("Invalid number format");
    let target = lines.next().unwrap().unwrap();
    assert_eq!(target.len(), target_size);

    // Output: result
    let result = min_valid_strings(words, target);
    println!("{}", result);
}