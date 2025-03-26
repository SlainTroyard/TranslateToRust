use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::BTreeSet;

const MOD: i64 = 1_070_777_777;

// Structure to hold a dynamic array of hashes for each prefix length
struct HashSet {
    data: BTreeSet<i64>,
}

impl HashSet {
    fn new() -> Self {
        HashSet { data: BTreeSet::new() }
    }

    fn add(&mut self, hash: i64) {
        self.data.insert(hash);
    }
}

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    800_000_000 + rng.gen_range(0..100_000_000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    // Seed the random number generator
    let mut rng = rand::thread_rng();

    // Step 1: Determine the length of the target string
    let n = target.len();

    // Step 2: Find the maximum length among all words
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    // Step 3: Initialize the base for hashing
    let base = get_random_base() as i64;

    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base = vec![1; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as i64) % MOD;
    }

    // Step 6: Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet> = vec![HashSet::new(); max_len];

    // Step 7: Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0;
        for j in 0..len {
            h = (h * base + word.as_bytes()[j] as i64) % MOD;
            sets[j].add(h);
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
            if nxt_r + 1 > n {
                break; // Safety check
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len {
                break; // Safety check
            }

            // Check if the hash exists in the HashSet for this prefix length
            if sets[prefix_len].data.contains(&current_hash) {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of words
    let words_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read the words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let _ = lines.next().unwrap()?; // Skip the length of the word
        words.push(lines.next().unwrap()?);
    }

    // Read the target string
    let _ = lines.next().unwrap()?; // Skip the length of the target
    let target = lines.next().unwrap()?;

    // Compute the result
    let res = min_valid_strings(&words, &target);

    // Print the result
    println!("{}", res);

    Ok(())
}