use std::io::{self, BufRead};
use std::collections::HashSet;
use rand::Rng;

const MOD: i64 = 1_070_777_777;

// Structure to hold a dynamic array of hashes for each prefix length
struct HashArray {
    data: Vec<i64>,
}

impl HashArray {
    fn new() -> Self {
        HashArray { data: Vec::new() }
    }

    fn add(&mut self, hash: i64) {
        self.data.push(hash);
    }

    fn sort_and_unique(&mut self) {
        self.data.sort_unstable();
        self.data.dedup();
    }

    fn binary_search(&self, target: i64) -> bool {
        self.data.binary_search(&target).is_ok()
    }
}

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u64 {
    let mut rng = rand::thread_rng();
    800_000_000 + rng.gen_range(0..100_000_000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    let base = get_random_base() as u64;

    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base = vec![1u64; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD as u64;
    }

    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash = vec![0u64; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD as u64;
    }

    // Step 6: Initialize HashArrays for each possible prefix length
    let mut sets: Vec<HashArray> = vec![HashArray::new(); max_len];

    // Step 7: Populate the HashArrays with prefix hashes from the words
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0u64;
        for j in 0..len {
            h = (h * base + word.as_bytes()[j] as u64) % MOD as u64;
            sets[j].add(h as i64);
        }
    }

    // Step 8: Sort and remove duplicates in each HashArray
    for set in &mut sets {
        set.sort_and_unique();
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
            let current_hash = (pre_hash[nxt_r + 1] + MOD as u64 - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD as u64) % MOD as u64;
            let sh = current_hash as i64;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len as usize {
                break; // Safety check
            }

            // Binary search in the sorted HashArray for this prefix length
            if sets[prefix_len].binary_search(sh) {
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

    // Read wordsSize
    let words_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Read words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let _ = lines.next().unwrap()?; // Skip the length of the word
        words.push(lines.next().unwrap()?);
    }

    // Read targetSize and target
    let _ = lines.next().unwrap()?; // Skip the length of the target
    let target = lines.next().unwrap()?;

    // Compute and print the result
    let res = min_valid_strings(&words, &target);
    println!("{}", res);

    Ok(())
}