use std::collections::HashSet;
use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const MOD: i64 = 1_070_777_777;

fn min_valid_strings(words: &Vec<String>, target: &str) -> i32 {
    let n = target.len();
    let target_bytes = target.as_bytes();

    // Seed the RNG using the current time (similar to chrono::steady_clock in C++)
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    // Generate a random BASE uniformly in [800_000_000, 900_000_000]
    let base: i64 = rng.gen_range(800_000_000..=900_000_000);

    // Precompute powers of base and prefix hashes for the target string.
    // We'll use two vectors: pow_base and pre_hash, both of size n+1.
    let mut pow_base = vec![0i64; n + 1];
    let mut pre_hash = vec![0i64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
        // target_bytes[i] is u8, we want to treat it as integer (like in C++ using target[i])
        pre_hash[i + 1] = (pre_hash[i] * base + target_bytes[i] as i64) % MOD;
    }

    // Closure to compute the hash of the substring target[l..r]
    let sub_hash = |l: usize, r: usize| -> i64 {
        // Using proper modulo arithmetic
        (pre_hash[r] - pre_hash[l] * pow_base[r - l] % MOD + MOD) % MOD
    };

    // Determine the maximum length of any word in the input.
    let mut max_len = 0;
    for w in words.iter() {
        max_len = max_len.max(w.len());
    }

    // Create a vector of HashSet to store hash values for each possible prefix length.
    // sets[j] will store hash values for all words for which a prefix of length j+1 exists.
    let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
    for w in words.iter() {
        let w_bytes = w.as_bytes();
        let mut h = 0i64;
        for (j, &b) in w_bytes.iter().enumerate() {
            h = (h * base + b as i64) % MOD;
            // Insert the hash into the corresponding set if j < max_len.
            if j < max_len {
                sets[j].insert(h);
            }
        }
    }

    // Closure to compute the maximum jump (the maximum valid substring length from index i)
    let max_jump = |i: usize| -> usize {
        // The substring length cannot exceed either n-i or max_len.
        let max_possible = (n - i).min(max_len);
        let mut left = 0usize;
        let mut right = max_possible + 1; // right is exclusive upper bound.
        // Binary search: find the maximum valid substring length where the hash is in the set.
        while left + 1 < right {
            let mid = (left + right) / 2;
            // Check if the substring target[i..i+mid] is in the corresponding set.
            if sets[mid - 1].contains(&sub_hash(i, i + mid)) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    };

    // Greedy approach to cover the entire target string with valid segments
    let mut ans = 0;
    let mut cur_r = 0usize;
    let mut nxt_r = 0usize;
    for i in 0..n {
        // For each index, update nxt_r using the maximum jump from i.
        nxt_r = nxt_r.max(i + max_jump(i));
        if i == cur_r {
            // If we are forced to pick a new segment but no extension is possible, return -1.
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
    // Set up the standard input and output.
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();
    // Read the entire input into a string and then iterate over tokens.
    stdin_lock.read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    // Parse the number of words.
    let words_size: usize = tokens.next().unwrap().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    // For each word, first get word_len then the word itself.
    // Although word_len is provided, we don't need to validate it.
    for _ in 0..words_size {
        let _word_len: usize = tokens.next().unwrap().parse().unwrap();
        let word = tokens.next().unwrap().to_string();
        words.push(word);
    }
    // Read target length and target string.
    let _target_len: usize = tokens.next().unwrap().parse().unwrap();
    let target = tokens.next().unwrap().to_string();

    // Compute the answer using our function.
    let ans = min_valid_strings(&words, &target);

    // Print the answer.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", ans)?;
    Ok(())
}