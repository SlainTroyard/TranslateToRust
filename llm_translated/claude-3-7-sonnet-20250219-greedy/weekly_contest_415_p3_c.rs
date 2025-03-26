use std::collections::HashSet;
use std::io::{self, BufRead};
use rand::Rng;

// Define the modulus as per the C code
const MOD: u64 = 1070777777;

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u32 {
    let mut rng = rand::thread_rng();
    800_000_000 + rng.gen_range(0..100_000_000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    // Step 1: Determine the length of the target string
    let n = target.len();
    
    // Step 2: Find the maximum length among all words
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    
    // Step 3: Initialize the base for hashing
    let base = get_random_base() as u64;
    
    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base = vec![1u64; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }
    
    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash = vec![0u64; n + 1];
    let target_bytes = target.as_bytes();
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target_bytes[i] as u64) % MOD;
    }
    
    // Step 6: Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); max_len];
    
    // Step 7: Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len().min(max_len);
        let word_bytes = word.as_bytes();
        let mut h = 0u64;
        for j in 0..len {
            h = (h * base + word_bytes[j] as u64) % MOD;
            sets[j].insert(h as i32);
        }
    }
    
    // Step 9: Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;
    
    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from the current position i
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..nxt_r+1)
            if nxt_r + 1 > n { break; } // Safety check
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as i32;
            
            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len { break; } // Safety check
            
            // Check if the hash exists in the set for this prefix length
            if sets[prefix_len].contains(&sh) {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read words size
    let words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        // Read word length (we don't actually need this in Rust, but we need to consume it)
        let _words_col_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        // Read the actual word
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }
    
    // Read target size
    let _target_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read target
    let target = lines.next().unwrap().unwrap();
    
    // Calculate result
    let res = min_valid_strings(&words, &target);
    
    // Print result
    println!("{}", res);
}