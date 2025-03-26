use std::io::{self, BufRead};
use std::process;
use rand::Rng;

// Define the modulus as per the C++ code
const MOD: i64 = 1070777777;

// Structure to hold a dynamic array of hashes for each prefix length
struct HashSet {
    data: Vec<i32>, // Array of hash values
}

impl HashSet {
    // Initialize a HashSet
    fn new() -> Self {
        HashSet {
            data: Vec::new(),
        }
    }

    // Add a hash value to the HashSet
    fn add_hash(&mut self, hash: i32) {
        self.data.push(hash);
    }

    // Sort the HashSet and remove duplicates
    fn sort_and_unique(&mut self) {
        if self.data.is_empty() {
            return;
        }
        self.data.sort_unstable();
        self.data.dedup();
    }
}

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u32 {
    let mut rng = rand::thread_rng();
    800000000 + rng.gen_range(0..100000000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    // Seed the random number generator - Rust rand crate handles this automatically per thread

    // Step 1: Determine the length of the target string
    let n = target.len();

    // Step 2: Find the maximum length among all words
    let mut max_len = 0;
    for word in words {
        if word.len() > max_len {
            max_len = word.len();
        }
    }

    // Step 3: Initialize the base for hashing
    let base = get_random_base() as i64;

    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base = vec![0_i64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash = vec![0_i64; n + 1];
    pre_hash[0] = 0;
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as i64) % MOD;
    }

    // Step 6: Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet> = Vec::with_capacity(max_len);
    for _ in 0..max_len {
        sets.push(HashSet::new());
    }

    // Step 7: Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = std::cmp::min(word.len(), max_len);
        let mut h = 0_i64;
        for j in 0..len {
            h = (h * base + word.as_bytes()[j] as i64) % MOD;
            sets[j].add_hash(h as i32);
        }
    }

    // Step 8: Sort and remove duplicates in each HashSet
    for j in 0..max_len {
        sets[j].sort_and_unique();
    }

    // Step 9: Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from the current position i
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..nxt_r+1)
            if nxt_r + 1 > n {
                break; // Safety check
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as i32;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len {
                break; // Safety check
            }

            // Binary search in the sorted HashSet for this prefix length
            let set = &sets[prefix_len];
            let mut left = 0;
            let mut right = set.data.len() as i32 - 1;
            let mut found = false;
            while left <= right {
                let mid = left + (right - left) / 2;
                if set.data[mid as usize] == sh {
                    found = true;
                    break;
                } else if set.data[mid as usize] < sh {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            if found {
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

    let words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut words: Vec<String> = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let _word_col_size: usize = parts.next().unwrap().parse().unwrap();
        let word = parts.next().unwrap().to_string();
        words.push(word);
    }

    let target_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let target = lines.next().unwrap().unwrap().trim().to_string();

    let res = min_valid_strings(&words, &target);
    println!("{}", res);
}