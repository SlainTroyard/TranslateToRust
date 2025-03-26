use std::collections::BTreeSet;
use std::io::{self, BufRead, Write};

const MOD: u64 = 1_070_777_777;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of words
    let words_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut words: Vec<String> = Vec::with_capacity(words_size);

    // Read the words
    for _ in 0..words_size {
        let word: String = lines.next().unwrap().unwrap();
        words.push(word);
    }

    // Read the target string
    let target: String = lines.next().unwrap().unwrap();

    // Call the function to compute the minimum number of valid strings
    let result = min_valid_strings(&words, &target);

    // Print the result
    println!("{}", result);
}

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

    // Seed the random number generator
    let base: u64 = 800_000_000 + (rand::random::<u32>() as u64 % 100_000_000);

    // Precompute powers of the base modulo MOD
    let pow_base: Vec<u64> = (0..=n).map(|i| mod_pow(base, i as u64, MOD)).collect();

    // Compute prefix hashes for the target string
    let pre_hash: Vec<u64> = (0..=n).map(|i| {
        target[0..i].chars().fold(0, |acc, c| (acc * base + c as u64) % MOD)
    }).collect();

    // Initialize HashSets for each possible prefix length
    let mut sets: Vec<BTreeSet<u64>> = vec![BTreeSet::new(); max_len];

    // Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len();
        let mut h: u64 = 0;
        for (j, c) in word.chars().enumerate() {
            h = (h * base + c as u64) % MOD;
            sets[j].insert(h);
        }
    }

    // Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from the current position i
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..=nxt_r]
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[(nxt_r + 1 - i) as usize]) % MOD) % MOD;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len >= max_len {
                break;
            }

            // Binary search in the sorted HashSet for this prefix length
            if sets[prefix_len].contains(&current_hash) {
                nxt_r += 1;
            } else {
                break;
            }
        }

        // If we've reached the end of the current bridge, update the bridge
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

// Helper function to compute (base^exp) % mod
fn mod_pow(mut base: u64, mut exp: u64, mod_val: u64) -> u64 {
    if exp == 0 {
        return 1;
    }
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod_val;
        }
        exp /= 2;
        base = (base * base) % mod_val;
    }
    result
}