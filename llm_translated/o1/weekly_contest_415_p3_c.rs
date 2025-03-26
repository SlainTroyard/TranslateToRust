// Weekly Contest 415 Problem 3 in Rust
// Requirements:
// 1. Translate the entire C program to Rust, including main(), preserving the exact I/O format.
// 2. Preserve the algorithm's logic exactly.
// 3. Use idiomatic Rust with proper error handling.
// 4. Maintain the exact same stdin/stdout format as the original C code.
// 5. Do not assume any changes to the input format; replicate the same scanning logic.

use std::io::{self, BufRead};

// A global constant for the modulus, matching the C code's #define
const MOD: u64 = 1070777777;

// Function to simulate the getRandomBase() from the C code.
// Generates a random base within [800000000, 900000000).
// We use the "rand" crate for randomness. In a real environment,
// you may need to add rand = "0.8" (or similar) to Cargo.toml.
fn get_random_base() -> u64 {
    // Use thread_rng for a random u64 within the range
    use rand::Rng;
    let mut rng = rand::thread_rng();
    800000000 + rng.gen_range(0..100000000) as u64
}

// minValidStrings: implements the same algorithm as the C version.
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len(); // length of target
    if n == 0 {
        return 0; // edge case: if target is empty, no strings needed
    }

    // Find max_len among all words
    let mut max_len = 0;
    for w in words {
        let len_w = w.len();
        if len_w > max_len {
            max_len = len_w;
        }
    }

    // Get a random base
    let base = get_random_base();

    // Precompute powers of the base modulo MOD
    let mut pow_base = vec![0u64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        let mul_val = (pow_base[i] as u128 * base as u128) % (MOD as u128);
        pow_base[i + 1] = mul_val as u64;
    }

    // Compute prefix hashes for the target
    let mut pre_hash = vec![0u64; n + 1];
    for (i, c) in target.bytes().enumerate() {
        let mul_val = (pre_hash[i] as u128 * base as u128) % (MOD as u128);
        let add_val = (mul_val + c as u128) % (MOD as u128);
        pre_hash[i + 1] = add_val as u64;
    }

    // Create containers (Vec<Vec<i32>>) to mimic HashSet for each prefix length
    let mut sets: Vec<Vec<i32>> = vec![Vec::new(); max_len];

    // Populate the sets with prefix hashes for each word
    for w in words {
        let mut h: u64 = 0;
        let len_w = w.len();
        for (j, c) in w.bytes().enumerate() {
            if j >= max_len {
                break;
            }
            let mul_val = (h as u128 * base as u128) % (MOD as u128);
            let add_val = (mul_val + c as u128) % (MOD as u128);
            h = add_val as u64;
            sets[j].push(h as i32);
        }
    }

    // Sort and remove duplicates in each set
    for j in 0..max_len {
        let v = &mut sets[j];
        v.sort_unstable();
        v.dedup();
    }

    // Two-pointer approach
    let mut ans: i32 = 0;
    let mut cur_r: usize = 0;
    let mut nxt_r: usize = 0;

    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from position i
        while nxt_r < n && (nxt_r - i) < max_len {
            // substring hash from target[i..(nxt_r+1)]
            let length = nxt_r + 1 - i;
            // (pre_hash[nxt_r+1] + MOD - (pre_hash[i] * pow_base[length]) % MOD) % MOD
            let mut current_hash = pre_hash[nxt_r + 1];
            let sub_val = ((pre_hash[i] as u128) * (pow_base[length] as u128)) % (MOD as u128);
            if sub_val as u64 > current_hash {
                current_hash = (current_hash + MOD - sub_val as u64) % MOD;
            } else {
                current_hash = (current_hash - sub_val as u64) % MOD;
            }
            let sh = current_hash as i32;

            // Binary search in sets[length - 1], except length might be 0-based index
            let prefix_len_index = length - 1; // Because sets[j] is for j in 0..max_len
            if prefix_len_index >= max_len {
                break;
            }

            let found = sets[prefix_len_index].binary_search(&sh).is_ok();
            if found {
                nxt_r += 1;
            } else {
                break;
            }
        }

        // If we've reached the end of the current coverage, we need to update
        if i == cur_r {
            // If no extension was possible, return -1
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
    // We will read lines from stdin, parse the values,
    // and replicate the exact format of the original C code.

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 1) Read wordsSize
    let words_size_str = lines
        .next()
        .expect("Expected to read wordsSize")
        .expect("Failed to read line");
    let words_size = words_size_str
        .trim()
        .parse::<usize>()
        .expect("wordsSize must be an integer");

    // 2) Read each word:
    //    For each i in 0..words_size:
    //      - read wordsColSize
    //      - read the word
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let col_size_str = lines
            .next()
            .expect("Expected to read wordsColSize")
            .expect("Failed to read line");
        let _col_size = col_size_str
            .trim()
            .parse::<usize>()
            .expect("wordsColSize must be an integer");

        let word_str = lines
            .next()
            .expect("Expected to read a word")
            .expect("Failed to read line");
        let word = word_str.trim().to_string();
        words.push(word);
    }

    // 3) Read targetSize
    let target_size_str = lines
        .next()
        .expect("Expected to read targetSize")
        .expect("Failed to read line");
    let _target_size = target_size_str
        .trim()
        .parse::<usize>()
        .expect("targetSize must be an integer");

    // 4) Read target
    let target_str = lines
        .next()
        .expect("Expected to read target string")
        .expect("Failed to read line");
    let target = target_str.trim();

    // Call the main algorithm function
    let res = min_valid_strings(&words, target);

    // Print the result
    println!("{}", res);
}