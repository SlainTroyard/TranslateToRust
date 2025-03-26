use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

// Define the modulus as per the C code
const MOD: u64 = 1070777777;

// Structure to hold a dynamic array of hashes for each prefix length
struct HashSet {
    data: Vec<i32>,
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
    
    // Size of the HashSet
    fn size(&self) -> usize {
        self.data.len()
    }
}

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u32 {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);
    800_000_000 + rng.gen_range(0..100_000_000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[String], words_size: usize, target: &str) -> i32 {
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
    for (i, &c) in target.as_bytes().iter().enumerate() {
        pre_hash[i + 1] = (pre_hash[i] * base + c as u64) % MOD;
    }
    
    // Step 6: Initialize HashSets for each possible prefix length
    let mut sets = vec![HashSet::new(); max_len];
    
    // Step 7: Populate the HashSets with prefix hashes from the words
    for i in 0..words_size {
        let len = std::cmp::min(words[i].len(), max_len);
        let mut h = 0u64;
        for (j, &c) in words[i].as_bytes().iter().take(len).enumerate() {
            h = (h * base + c as u64) % MOD;
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
            if nxt_r + 1 > n { break; } // Safety check
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as i32;
            
            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len >= max_len { break; } // Safety check
            
            // Binary search in the sorted HashSet for this prefix length
            let set = &sets[prefix_len];
            let mut found = false;
            let mut left = 0;
            let mut right = set.size().saturating_sub(1);
            
            while left <= right && right < set.size() {
                let mid = left + (right - left) / 2;
                if set.data[mid] == sh {
                    found = true;
                    break;
                } else if set.data[mid] < sh {
                    left = mid + 1;
                } else {
                    if mid == 0 { break; }
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of words
    let words_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let words_col_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
        let word = lines.next().unwrap()?;
        words.push(word);
    }
    
    // Read the target
    let target_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let target = lines.next().unwrap()?;
    
    // Compute the result
    let res = min_valid_strings(&words, words_size, &target);
    
    // Print the result
    println!("{}", res);
    
    Ok(())
}