use std::io::{self, BufRead, Write};

const MOD: u64 = 1070777777;

// Structure to hold a dynamic array of hashes for each prefix length
struct HashSet {
    data: Vec<i32>,
}

impl HashSet {
    // Initialize a HashSet
    fn new() -> Self {
        HashSet { data: Vec::new() }
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
    use rand::Rng;
    let mut rng = rand::thread_rng();
    800_000_000 + rng.gen_range(0..100_000_000)
}

// The main function to compute the minimum number of valid strings
fn min_valid_strings(words: &[&str], target: &str) -> i32 {
    // Step 1: Determine the length of the target string
    let n = target.len();

    // Step 2: Find the maximum length among all words
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

    // Step 3: Initialize the base for hashing
    let base = get_random_base();

    // Step 4: Precompute powers of the base modulo MOD
    let mut pow_base = vec![1; n + 1];
    for i in 1..=n {
        pow_base[i] = (pow_base[i - 1] * base as u64) % MOD;
    }

    // Step 5: Compute prefix hashes for the target string
    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base as u64 + target.as_bytes()[i] as u64) % MOD;
    }

    // Step 6: Initialize HashSets for each possible prefix length
    let mut sets: Vec<HashSet> = (0..max_len).map(|_| HashSet::new()).collect();

    // Step 7: Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len();
        let mut h = 0;
        for (j, &ch) in word.as_bytes().iter().enumerate().take(len) {
            h = (h * base as u64 + ch as u64) % MOD;
            sets[j].add_hash(h as i32);
        }
    }

    // Step 8: Sort and remove duplicates in each HashSet
    for set in sets.iter_mut() {
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
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as i32;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len {
                break; // Safety check
            }

            // Binary search in the sorted HashSet for this prefix length
            let set = &sets[prefix_len];
            let found = set.data.binary_search(&sh).is_ok();

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
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read the number of words
    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let words_size: usize = input.trim().parse().unwrap();

    // Read the words
    let mut words: Vec<String> = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let words_col_size: usize = input.trim().parse().unwrap();
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        words.push(input.trim().to_string());
    }

    // Read the target
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let target_size: usize = input.trim().parse().unwrap();
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let target = input.trim().to_string();

    // Compute the result
    let res = min_valid_strings(&words, &target);

    // Output the result
    writeln!(stdout_lock, "{}", res).unwrap();
}