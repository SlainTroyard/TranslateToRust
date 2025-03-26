// Translation of the given C solution into idiomatic Rust.
// Preserves the exact algorithm and I/O format, using Rust's safety features.
//
// Problem: LeetCode Weekly Contest 415 Problem 4
// Original C code included dynamic memory allocations, random base generation,
// power array computation, prefix hashing, multi-HashSet usage, and a two-pointer
// approach. This Rust version mimics the logic exactly, while using idiomatic
// Rust constructs (e.g., Vec for dynamic arrays).

use std::io::{stdin, stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;

/// The modulus from the original code.
const MOD: u64 = 1070777777;

/// A simple wrapper struct to store a dynamic list of i32 hash values.
/// In the C code, this was "HashSet" storing "int* data" plus size/capacity.
/// Here, we simply use a Vec<i32>.
#[derive(Default)]
struct HashSetWrapper {
    data: Vec<i32>,
}

impl HashSetWrapper {
    /// Creates a new, empty HashSetWrapper.
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Adds a hash value to the underlying Vec.
    /// Automatically grows as needed, like C code with realloc.
    fn add_hash(&mut self, hash: i32) {
        self.data.push(hash);
    }

    /// Sorts the stored hash values and removes duplicates,
    /// matching the C code's qsort + unique procedure.
    fn sort_and_unique(&mut self) {
        if self.data.is_empty() {
            return;
        }
        self.data.sort();
        self.data.dedup();
    }
}

/// Generates a random base value between 800,000,000 and 899,999,999,
/// mimicking the C code's getRandomBase() behavior.
/// In C: 800000000 + (rand() % 100000000).
fn get_random_base() -> u64 {
    // Seed with system time to mimic srand((unsigned int)time(NULL)).
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng = StdRng::seed_from_u64(seed);
    800_000_000_u64 + rng.gen_range(0..100_000_000_u64)
}

/// A helper function to safely multiply two u64 values under modulo,
/// avoiding potential 64-bit overflow by using 128-bit intermediate.
fn mod_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % (MOD as u128)) as u64
}

/// Translated version of the C minValidStrings function.
///
/// 1. Precompute powers of the base (pow_base).
/// 2. Compute prefix hashes of the target (pre_hash).
/// 3. Build HashSetWrapper arrays for prefix lengths from all words.
/// 4. Sort/deduplicate them.
/// 5. Use a two-pointer approach to find the minimum number of substrings.
///
/// Returns the minimum count of valid substrings or -1 if impossible.
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    // Step 1: Determine the length of the target string
    let n = target.len();

    // Step 2: Find the maximum length among all words
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

    // Step 3: Initialize the base for hashing
    let base = get_random_base();

    // Step 4: Precompute powers of the base modulo MOD (pow_base)
    //    pow_base[i] = base^i mod MOD
    let mut pow_base = vec![0u64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = mod_mul(pow_base[i], base);
    }

    // Step 5: Compute prefix hashes for the target string (pre_hash)
    //    pre_hash[i + 1] = hash of target[..=i]
    let mut pre_hash = vec![0u64; n + 1];
    for (i, &byte_char) in target.as_bytes().iter().enumerate() {
        let val = byte_char as u64;
        pre_hash[i + 1] = (mod_mul(pre_hash[i], base) + val) % MOD;
    }

    // Step 6: Initialize HashSetWrapper for each possible prefix length
    let mut sets: Vec<HashSetWrapper> = Vec::with_capacity(max_len);
    for _ in 0..max_len {
        sets.push(HashSetWrapper::new());
    }

    // Step 7: Populate the HashSetWrappers with prefix hashes from each word
    for word in words {
        let mut h: u64 = 0;
        let length = word.len().min(max_len);
        for (j, &byte_char) in word.as_bytes()[..length].iter().enumerate() {
            let val = byte_char as u64;
            h = (mod_mul(h, base) + val) % MOD;
            sets[j].add_hash(h as i32);
        }
    }

    // Step 8: Sort and remove duplicates in each HashSet
    for set in &mut sets {
        set.sort_and_unique();
    }

    // Step 9: Two-pointer approach to find the minimum number of valid segments
    let mut ans = 0;
    let mut cur_r = 0; // current bridge endpoint
    let mut nxt_r = 0; // next bridge endpoint

    // i iterates over each starting position
    for i in 0..n {
        // Extend nxt_r as far as possible from position i
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of substring target[i..nxt_r+1)
            // pre_hash is 1-based, so for substring i..nxt_r, we do:
            // current_hash = (pre_hash[nxt_r+1] + MOD - pre_hash[i] * pow_base[nxt_r+1 - i]) % MOD
            if nxt_r + 1 > n {
                break;
            }
            let sub_len = nxt_r + 1 - i;
            let hash_val = {
                let mul_part = mod_mul(pre_hash[i], pow_base[sub_len]);
                (pre_hash[nxt_r + 1].wrapping_add(MOD)
                    .wrapping_sub(mul_part))
                    % MOD
            } as i32;

            // prefix_len for sets index
            let prefix_len = sub_len - 1; // zero-based index for sets
            if prefix_len < 0 || prefix_len as usize >= max_len {
                break;
            }

            // Binary search for hash_val in sets[prefix_len]
            let set_data = &sets[prefix_len as usize].data;
            if set_data.binary_search(&hash_val).is_ok() {
                nxt_r += 1;
            } else {
                break;
            }
        }

        // If we've reached the end of the current bridge, update the bridge
        if i == cur_r {
            if i == nxt_r {
                // No progress can be made, return -1
                return -1;
            }
            cur_r = nxt_r;
            ans += 1;
        }
    }

    ans
}

/// Reads a single integer from stdin, trimming and parsing to usize.
fn read_usize() -> usize {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().parse().expect("Failed to parse integer")
}

/// Reads a single line from stdin as a String (trimmed).
fn read_string() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().to_string()
}

fn main() {
    // Match C code's I/O behavior:
    // 1) Read wordsSize
    // 2) For each word: read wordsColSize, then read the word
    // 3) Read targetSize
    // 4) Read target
    // 5) Compute result and print
    //
    // Example input flow in C:
    // int wordsSize;
    // scanf("%d", &wordsSize);
    // for i in 0..wordsSize:
    //     scanf("%d", &wordsColSize);
    //     scanf("%s", words[i]);
    // int targetSize;
    // scanf("%d", &targetSize);
    // scanf("%s", target);

    let words_size = read_usize();
    let mut words = Vec::with_capacity(words_size);

    for _ in 0..words_size {
        // Read the length, but we only need it to know how many chars to read next.
        let _words_col_size = read_usize();
        // Read the actual word
        let w = read_string();
        words.push(w);
    }

    // Read target size
    let _target_size = read_usize();
    // Read target string
    let target = read_string();

    // Compute the result
    let res = min_valid_strings(&words, &target);

    // Print the result (same as printf("%d\n", res);)
    println!("{}", res);

    // Flush stdout to match typical C program behavior
    stdout().flush().unwrap();
}