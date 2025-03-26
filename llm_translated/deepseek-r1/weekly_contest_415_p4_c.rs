use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD: u64 = 1_070_777_777;

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    if n == 0 {
        return 0;
    }

    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);
    if max_len == 0 {
        return -1;
    }

    // Generate random base using current time as seed
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng = StdRng::seed_from_u64(seed);
    let base: u32 = rng.gen_range(800_000_000..900_000_000);

    // Precompute powers of the base modulo MOD
    let mut pow_base = vec![0u64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base as u64) % MOD;
    }

    // Compute prefix hashes for target
    let target_bytes = target.as_bytes();
    let mut pre_hash = vec![0u64; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base as u64 + target_bytes[i] as u64) % MOD;
    }

    // Build hash sets for each prefix length
    let mut sets = vec![vec![]; max_len];
    for word in words {
        let len = word.len().min(max_len);
        let mut hash = 0u64;
        for (j, &c) in word.as_bytes().iter().take(len).enumerate() {
            hash = (hash * base as u64 + c as u64) % MOD;
            sets[j].push(hash);
        }
    }

    // Sort and deduplicate each hash set
    for set in &mut sets {
        set.sort_unstable();
        set.dedup();
    }

    // Two-pointer approach to find minimum valid strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        // Extend as far as possible from current position
        while nxt_r < n && (nxt_r - i) < max_len {
            let substring_len = nxt_r + 1 - i;
            let prefix_len = nxt_r - i;
            let pow = pow_base[substring_len];
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow) % MOD) % MOD;

            if let Ok(_) = sets[prefix_len].binary_search(&current_hash) {
                nxt_r += 1;
            } else {
                break;
            }
        }

        // Update current range and answer
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
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read words
    let words_size: usize = tokens.next().unwrap().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let _ = tokens.next().unwrap().parse::<usize>().unwrap(); // length ignored
        let word = tokens.next().unwrap().to_string();
        words.push(word);
    }

    // Read target
    let _ = tokens.next().unwrap().parse::<usize>().unwrap(); // length ignored
    let target = tokens.next().unwrap().to_string();

    println!("{}", min_valid_strings(&words, &target));
}