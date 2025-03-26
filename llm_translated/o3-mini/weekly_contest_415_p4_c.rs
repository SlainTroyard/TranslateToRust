use std::io::{self, Read};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

// Bring in the random number generator from the "rand" crate
// Add this crate in Cargo.toml: rand = "0.8"
use rand::Rng;

// Define the modulus as per the original code
const MOD: u64 = 1070777777;

// This function computes the minimum number of valid strings using the algorithm from the C code.
// It returns -1 if no valid segmentation exists.
// words: slice of words (each a String)
// target: the target string
fn min_valid_strings(words: &[String], target: &str) -> i32 {
    // Step 1: Determine the length of the target string.
    let n = target.len();
    let target_bytes = target.as_bytes();

    // Step 2: Find the maximum length among all words.
    let mut max_len = 0;
    for word in words {
        let len = word.len();
        if len > max_len {
            max_len = len;
        }
    }
    // If no words exist, it's impossible to form the target.
    if max_len == 0 {
        return -1;
    }

    // Step 3: Initialize the base for hashing.
    // Seed the RNG using the system time.
    // Note: The C code seeds with time(NULL)
    let mut rng = {
        // Use current time in nanos as seed for better randomness.
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|err| {
                eprintln!("SystemTime error: {}", err);
                process::exit(1);
            })
            .as_nanos() as u64;
        // We simply create the thread_rng; the actual seed is handled internally.
        rand::thread_rng()
    };
    let base: u32 = 800_000_000 + rng.gen_range(0..100_000_000); // random base in [8e8, 9e8)

    // Step 4: Precompute powers of the base modulo MOD.
    let mut pow_base: Vec<u64> = vec![0; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * (base as u64)) % MOD;
    }

    // Step 5: Compute prefix hashes for the target string.
    let mut pre_hash: Vec<u64> = vec![0; n + 1];
    for i in 0..n {
        // Using target_bytes[i] as u8 (which is similar to unsigned char)
        pre_hash[i + 1] = (pre_hash[i] * (base as u64) + target_bytes[i] as u64) % MOD;
    }

    // Step 6: Initialize HashSets. In Rust we'll use Vec of Vec<i32>.
    // Each set at index j will hold the unique hash values for prefixes of length j+1 from the words.
    let mut sets: Vec<Vec<i32>> = vec![Vec::new(); max_len];

    // Step 7: Populate the HashSets with prefix hashes from the words.
    for word in words {
        let word_bytes = word.as_bytes();
        // Only consider up to max_len characters.
        let len = if word.len() < max_len { word.len() } else { max_len };
        let mut h: u64 = 0;
        for j in 0..len {
            h = (h * (base as u64) + word_bytes[j] as u64) % MOD;
            // Push the current hash (cast to i32, safe because MOD < 2^31)
            sets[j].push(h as i32);
        }
    }

    // Step 8: Sort and remove duplicates in each HashSet.
    for set in &mut sets {
        // Only sort non-empty sets.
        if !set.is_empty() {
            set.sort_unstable();
            set.dedup();
        }
    }

    // Step 9: Implement the two-pointer approach to find the minimum number of strings.
    let mut ans = 0;
    // cur_r: the farthest reached by the previous string segments.
    let mut cur_r = 0;
    // nxt_r: the farthest reachable so far by extending the current segment.
    let mut nxt_r = 0;

    // Iterate over each starting index i of the target.
    for i in 0..n {
        // Try to extend the segment as far as possible.
        // Ensure that the length of the considered substring (nxt_r - i + 1) does not exceed max_len.
        while nxt_r < n && (nxt_r - i) < max_len {
            // Compute the hash of the substring target[i..nxt_r+1]
            // Using the formula: hash(target[i..nxt_r+1]) = (pre_hash[nxt_r+1] - pre_hash[i] * pow_base[nxt_r+1-i]) mod MOD
            let len_sub = nxt_r + 1 - i;
            let sub_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[len_sub]) % MOD) % MOD;
            let sh = sub_hash as i32;

            // The prefix length index into 'sets' should be (nxt_r - i).
            // Note: sets[j] contains hashes for words of length j+1.
            let prefix_index = nxt_r - i;
            if prefix_index >= max_len {
                break; // Safety check: this should not happen due to while condition.
            }
            let set = &sets[prefix_index];
            // Do a binary search for the hash in the sorted vector.
            if set.binary_search(&sh).is_ok() {
                nxt_r += 1;
            } else {
                break;
            }
        }
        // If we've reached the end of the current segment boundary, update the segment.
        if i == cur_r {
            if i == nxt_r {
                // Cannot extend further, so segmentation fails.
                return -1;
            }
            cur_r = nxt_r;
            ans += 1;
        }
    }
    ans
}

fn main() {
    // Read the entire stdin into a string.
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", err);
        process::exit(1);
    }
    let mut tokens = input.split_whitespace();

    // Parse the number of words.
    let words_size: usize = match tokens.next() {
        Some(tok) => tok.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing wordsSize");
            process::exit(1);
        }),
        None => {
            eprintln!("No input for wordsSize");
            process::exit(1);
        }
    };

    // Read each word.
    let mut words: Vec<String> = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        // First integer: the column size (length) of the word. We read it but do not use it further.
        let _words_col_size: usize = match tokens.next() {
            Some(tok) => tok.parse().unwrap_or_else(|_| {
                eprintln!("Error parsing wordsColSize");
                process::exit(1);
            }),
            None => {
                eprintln!("Not enough input for wordsColSize");
                process::exit(1);
            }
        };
        // Read the word string.
        let word = match tokens.next() {
            Some(tok) => tok.to_string(),
            None => {
                eprintln!("Not enough input for word");
                process::exit(1);
            }
        };
        words.push(word);
    }

    // Parse the target size.
    let target_size: usize = match tokens.next() {
        Some(tok) => tok.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing targetSize");
            process::exit(1);
        }),
        None => {
            eprintln!("No input for targetSize");
            process::exit(1);
        }
    };

    // Read the target string.
    // Although we have target_size, we simply read the next token as the target string.
    let target = match tokens.next() {
        Some(tok) => tok.to_string(),
        None => {
            eprintln!("No input for target string");
            process::exit(1);
        }
    };

    // Compute the result.
    let res = min_valid_strings(&words, &target);
    // Output the result exactly as expected.
    println!("{}", res);
}