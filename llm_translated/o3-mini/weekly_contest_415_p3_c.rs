use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD: u64 = 1070777777;

// A structure representing a dynamic array (hash set) of hash values.
struct HashSet {
    data: Vec<i32>,
}

impl HashSet {
    // Create a new HashSet.
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    // Add a hash value to the set.
    fn add_hash(&mut self, hash: i32) {
        self.data.push(hash);
    }

    // Sort the data and remove duplicate values.
    fn sort_and_unique(&mut self) {
        if self.data.is_empty() {
            return;
        }
        self.data.sort();
        self.data.dedup();
    }
}

/// Generate a "random" base between 800,000,000 and 900,000,000.
/// Instead of using an external crate for randomness, we use the current system time.
fn get_random_base() -> u32 {
    // Get seconds since UNIX epoch.
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    // Similar to: 800000000 + (rand() % 100000000)
    800_000_000 + ((now % 100_000_000) as u32)
}

/// The main function to compute the minimum number of valid strings.
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len(); // length of target string

    // Step 2: Find the maximum length among all words.
    let mut max_len = 0;
    for word in words {
        let len = word.len();
        if len > max_len {
            max_len = len;
        }
    }
    
    // If there are no words or max_len == 0 (should not happen as per constraints)
    if max_len == 0 {
        return -1;
    }
    
    // Step 3: Initialize the base for hashing.
    let base = get_random_base() as u64;
    
    // Step 4: Precompute powers of the base modulo MOD.
    let mut pow_base = vec![0u64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }
    
    // Step 5: Compute prefix hashes for the target string.
    // pre_hash[i] stores hash for the prefix target[0..i]
    let mut pre_hash = vec![0u64; n + 1];
    {
        // Convert target to bytes for easier processing.
        let bytes = target.as_bytes();
        for i in 0..n {
            pre_hash[i + 1] = (pre_hash[i] * base + bytes[i] as u64) % MOD;
        }
    }
    
    // Step 6: Initialize a vector of HashSets for each possible prefix length (0-indexed)
    // Note: index 0 corresponds to prefix length of 1 in the C code.
    let mut sets: Vec<HashSet> = Vec::with_capacity(max_len);
    for _ in 0..max_len {
        sets.push(HashSet::new());
    }
    
    // Step 7: Populate the HashSets with prefix hashes from the words
    for word in words {
        // Only consider up to max_len characters from each word.
        let len = std::cmp::min(word.len(), max_len);
        let bytes = word.as_bytes();
        let mut h = 0u64;
        for j in 0..len {
            h = (h * base + bytes[j] as u64) % MOD;
            // In the C code, the j-th prefix (0-indexed) is stored in sets[j]
            sets[j].add_hash(h as i32);
        }
    }
    
    // Step 8: Sort and remove duplicates in each HashSet.
    for set in &mut sets {
        set.sort_and_unique();
    }
    
    // Step 9: Use a two-pointer (or greedy) approach to find the minimum number of strings.
    let mut ans = 0;
    let mut cur_r = 0;   // current right-bound of our segmentation
    let mut nxt_r = 0;   // next right-bound we can extend to
    
    // We iterate over each starting index i in the target string.
    while cur_r < n {
        let i = cur_r;
        // Try to extend the current segment as far as possible.
        nxt_r = cur_r;
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..nxt_r+1]:
            // Using the formula: hash = (pre_hash[nxt_r+1] - pre_hash[i] * pow_base[nxt_r+1-i]) % MOD
            let sub_len = nxt_r + 1 - i;
            let mut current_hash = (pre_hash[nxt_r + 1] + MOD
                - (pre_hash[i] * pow_base[sub_len]) % MOD)
                % MOD;
            let sh = current_hash as i32;
            
            // Determine the corresponding index in sets.
            // NOTE: In the C code, the "prefix length" used for index is (nxt_r - i)
            // which equals sub_len - 1.
            let prefix_idx = nxt_r - i;
            if prefix_idx >= max_len {
                break; // out of sets range; should not happen because of while condition
            }
            
            // Use binary search on the sorted set to check if sh exists.
            if sets[prefix_idx].data.binary_search(&sh).is_ok() {
                nxt_r += 1;
            } else {
                break;
            }
        }
        
        // If we are unable to extend from the current position,
        // then it's impossible to cover the target string.
        if cur_r == nxt_r {
            return -1;
        }
        // Update current right-bound and increase the segment count.
        cur_r = nxt_r;
        ans += 1;
    }
    
    ans
}

/// A helper structure for scanning input tokens.
struct Scanner<B> {
    reader: B,
    buffer: Vec<String>,
}

impl<B: BufRead> Scanner<B> {
    /// Create a new Scanner from a given buffered reader.
    fn new(reader: B) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    /// Fetch the next token, reading a new line from the reader if necessary.
    fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            let bytes_read = self.reader.read_line(&mut input).expect("Failed read");
            if bytes_read == 0 {
                panic!("No more tokens");
            }
            // Split the line into tokens; reverse it to use pop efficiently.
            self.buffer = input
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}

fn main() {
    // Prepare standard input and output.
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new(stdin.lock());

    // Read the number of words.
    let words_size: usize = sc.token();
    let mut words: Vec<String> = Vec::with_capacity(words_size);
    
    // For each word, first read the integer word length (wordsColSize), then the word itself.
    // We ignore the given length value and rely on the string length.
    for _ in 0..words_size {
        let _word_col_size: usize = sc.token(); // read but not used
        let word: String = sc.token();
        words.push(word);
    }
    
    // Read the target string.
    let _target_size: usize = sc.token(); // read target size (not used explicitly)
    let target: String = sc.token();
    
    // Compute the result using min_valid_strings.
    let res = min_valid_strings(&words, &target);
    
    // Print the result exactly as in the C code.
    writeln!(out, "{}", res).expect("Failed to write output");
}