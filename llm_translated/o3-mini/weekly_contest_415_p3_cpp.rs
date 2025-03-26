use std::collections::HashSet;
use std::io::{self, Read, Write};
use rand::Rng; // external crate for random number generation

// This function implements the algorithm to compute the minimum number
// of valid strings required to build the target string.
fn min_valid_strings(words: &Vec<String>, target: &str) -> i32 {
    let n = target.len();
    let target_bytes = target.as_bytes();

    // Define MOD constant (as given in the original C++ code)
    let modulo: i64 = 1_070_777_777;

    // Initialize randomness using thread_rng (which is seeded automatically with entropy)
    let mut rng = rand::thread_rng();
    // Choose a random BASE in the range [800_000_000, 900_000_000] inclusive.
    let base: i64 = rng.gen_range(800_000_000..=900_000_000);

    // Precompute powers of base and prefix hash values for the target string.
    let mut pow_base = vec![0i64; n + 1];
    let mut pre_hash = vec![0i64; n + 1];
    pow_base[0] = 1;
    // Loop through each character (byte) in the target string.
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % modulo;
        pre_hash[i + 1] = (pre_hash[i] * base + target_bytes[i] as i64) % modulo;
    }
    // Closure to compute the hash value for a substring in target from index l (inclusive) to r (exclusive)
    let sub_hash = |l: usize, r: usize| -> i64 {
        ((pre_hash[r] - pre_hash[l] * pow_base[r - l]) % modulo + modulo) % modulo
    };

    // Determine the maximum length among all words.
    let mut max_len = 0;
    for w in words {
        max_len = max_len.max(w.len());
    }

    // Create a vector of hash sets, each set corresponding to a prefix length.
    // sets[i] will store all hash values of prefixes of length i+1 among the given words.
    let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];

    // For each word, compute its prefix hashes and store them in the corresponding set.
    for w in words {
        let mut h = 0i64;
        for (j, &byte) in w.as_bytes().iter().enumerate() {
            h = (h * base + byte as i64) % modulo;
            // It's safe to index sets[j] since j < w.len() and max_len is the maximum word length.
            sets[j].insert(h);
        }
    }

    // Closure to compute the maximum jump we can take starting at index 'i' in target.
    // This uses binary search on the length of the substring.
    let max_jump = |i: usize| -> usize {
        // The maximum possible length to check is the minimum of remaining target length and max_len.
        let max_possible = (n - i).min(max_len);
        let mut left = 0;
        let mut right = max_possible + 1; // set right as one more than maximum possible jump
        // Binary search: the invariant is that a jump of length 'left' is valid,
        // and any jump >= right is invalid.
        while left + 1 < right {
            let mid = (left + right) / 2;
            // Get the hash of the substring target[i..i+mid]
            let cur_sub_hash = sub_hash(i, i + mid);
            // Check if this hash exists in the set corresponding to prefix length 'mid - 1'
            if sets[mid - 1].contains(&cur_sub_hash) {
                left = mid; // substring of length mid is valid, try longer
            } else {
                right = mid; // not found, try shorter
            }
        }
        left
    };

    // Greedy approach to cover the target string using the valid substrings.
    let mut ans = 0;
    let mut cur_r = 0; // current right boundary (end index) that we can reach with the chosen segments
    let mut nxt_r = 0; // next right boundary while processing the target

    for i in 0..n {
        // Update the farthest we can reach starting at index i.
        nxt_r = nxt_r.max(i + max_jump(i));
        // When we reach the boundary of the current jump, we may need to jump again.
        if i == cur_r {
            // If no further progress can be made, it's impossible to construct target.
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
    // Read entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // Split the input by whitespace to process tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx = 0;

    // Parse the number of words.
    let words_size: usize = tokens[idx].parse().expect("Failed to parse words_size");
    idx += 1;

    // Read all the words. For each word, first its length is given (ignored) and then the word itself.
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        // Parse the length of the current word.
        let _word_len: usize = tokens[idx].parse().expect("Failed to parse word length");
        idx += 1;
        // Parse the actual word.
        let word = tokens[idx].to_string();
        idx += 1;
        words.push(word);
    }

    // Parse the target string input. First its length, then the string itself.
    let _target_len: usize = tokens[idx].parse().expect("Failed to parse target length");
    idx += 1;
    let target = tokens[idx].to_string();
    idx += 1;

    // Compute the answer using the min_valid_strings function.
    let result = min_valid_strings(&words, &target);

    // Write the result to stdout.
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}