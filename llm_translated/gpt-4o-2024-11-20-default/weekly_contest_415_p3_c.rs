use std::collections::HashSet;
use std::io::{self, BufRead};
use std::string::String;

// Define the modulus as per the C++ code
const MOD: u64 = 1070777777;

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(800_000_000..900_000_000)
}

// Function to compute the minimum number of valid strings
fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);
    let base = get_random_base();

    // Precompute powers of the base modulo MOD
    let mut pow_base = vec![1u64; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    // Compute prefix hashes for the target string
    let target_bytes = target.as_bytes();
    let mut pre_hash = vec![0u64; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = ((pre_hash[i] * base) + (target_bytes[i] as u64)) % MOD;
    }

    // Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet<u64>> = vec![HashSet::new(); max_len];

    // Populate the HashSets with prefix hashes from the words
    for word in &words {
        let word_bytes = word.as_bytes();
        let len = word.len();
        let mut h = 0u64;
        for j in 0..len {
            h = (h * base + (word_bytes[j] as u64)) % MOD;
            if j < max_len {
                sets[j].insert(h);
            }
        }
    }

    // Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from the current position i
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..nxt_r+1)
            if nxt_r + 1 > n {
                break;
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD
                - (pre_hash[i] * pow_base[nxt_r + 1 - i] % MOD))
                % MOD;

            let prefix_len = nxt_r - i;
            if prefix_len >= max_len {
                break;
            }

            // Check if the current_hash exists in the set for the prefix length
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

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let words_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse wordsSize");

    let mut words = Vec::new();
    for _ in 0..words_size {
        if let Some(Ok(word)) = lines.next() {
            words.push(word);
        }
    }

    let target = lines.next().unwrap().unwrap();

    // Call the min_valid_strings function
    let result = min_valid_strings(words, target);

    // Print the result
    println!("{}", result);
}