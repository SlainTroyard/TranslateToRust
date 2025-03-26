use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Read};

// Define the modulus as per the C++ code
const MOD: u64 = 1070777777;

// Structure to hold a dynamic array of hashes for each prefix length
#[derive(Debug)]
struct HashSet {
    data: Vec<u32>, // Changed to u32 to match the hash type
    size: usize,
    capacity: usize,
}

// Initialize a HashSet
impl HashSet {
    fn new() -> Self {
        HashSet {
            data: Vec::new(),
            size: 0,
            capacity: 0,
        }
    }

    // Add a hash value to the HashSet
    fn add_hash(&mut self, hash: u32) {
        if self.size == self.capacity {
            self.capacity = if self.capacity == 0 {
                16
            } else {
                self.capacity * 2
            };
            self.data.resize(self.capacity, 0);
        }
        if self.size < self.data.len() {
            self.data[self.size] = hash;
        } else {
            self.data.push(hash);
        }
        self.size += 1;
    }

    // Sort the HashSet and remove duplicates
    fn sort_and_unique(&mut self) {
        if self.size == 0 {
            return;
        }
        self.data[..self.size].sort();
        self.data[..self.size].dedup();
        self.size = self.data.len();
    }
}

// Function to generate a random base between 8e8 and 9e8
fn get_random_base() -> u32 {
    let mut rng = rand::thread_rng();
    800000000 + rng.gen_range(0..100000000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    // Step 1: Determine the length of the target string
    let n = target.len();

    // Step 2: Find the maximum length among all words
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    // Step 3: Initialize the base for hashing
    let base = get_random_base() as u64;

    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base: Vec<u64> = vec![0; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash: Vec<u64> = vec![0; n + 1];
    pre_hash[0] = 0;
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD;
    }

    // Step 6: Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet> = Vec::new();
    for _ in 0..max_len {
        sets.push(HashSet::new());
    }

    // Step 7: Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len().min(max_len);
        let mut h: u64 = 0;
        for j in 0..len {
            h = (h * base + word.as_bytes()[j] as u64) % MOD;
            sets[j].add_hash(h as u32);
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
            let sh = current_hash as u32;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len >= max_len {
                break; // Safety check
            }

            // Binary search in the sorted HashSet for this prefix length
            let set = &sets[prefix_len];
            let mut left = 0;
            let mut right = set.data.len().saturating_sub(1); // fix potential underflow
            let mut found = false;

            while left <= right {
                let mid = left + (right - left) / 2;
                if set.data[mid] == sh {
                    found = true;
                    break;
                } else if set.data[mid] < sh {
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let words_size: usize = lines.next().unwrap().parse().unwrap();
    let mut words: Vec<String> = Vec::new();

    for _ in 0..words_size {
        let words_col_size: usize = lines.next().unwrap().parse().unwrap();
        let word = lines.next().unwrap().to_string();
        words.push(word);
    }

    let target_size: usize = lines.next().unwrap().parse().unwrap();
    let target = lines.next().unwrap().to_string();

    let res = min_valid_strings(&words, &target);
    println!("{}", res);

    Ok(())
}