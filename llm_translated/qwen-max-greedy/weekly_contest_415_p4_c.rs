use std::io::{self, BufRead, Write};
use std::collections::BTreeSet;

const MOD: u64 = 1_070_777_777;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut words: Vec<String> = Vec::new();
    let mut target: String = String::new();

    // Read the number of words
    let words_size: usize = {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    // Read the words
    for _ in 0..words_size {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        words.push(input.trim().to_string());
    }

    // Read the target size and target string
    let target_size: usize = {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        target = input.trim().to_string();
    }

    // Call the main function and print the result
    let res = min_valid_strings(&words, &target);
    writeln!(stdout, "{}", res).unwrap();
}

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

    // Seed the random number generator
    let base = 800_000_000 + (rand::random::<u32>() % 100_000_000) as u64;

    // Precompute powers of the base modulo MOD
    let mut pow_base = vec![1; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    // Compute prefix hashes for the target string
    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD;
    }

    // Initialize HashSets for each possible prefix length
    let mut sets: Vec<BTreeSet<u64>> = vec![BTreeSet::new(); max_len];

    // Populate the HashSets with prefix hashes from the words
    for word in words {
        let len = word.len();
        let mut h = 0;
        for (j, &ch) in word.as_bytes().iter().enumerate() {
            h = (h * base + ch as u64) % MOD;
            sets[j].insert(h);
        }
    }

    // Implement the two-pointer approach to find the minimum number of strings
    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        // Attempt to extend nxt_r as far as possible from the current position i
        while nxt_r < n && (nxt_r - i) < max_len {
            if nxt_r + 1 > n {
                break;
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as u64;

            // Determine the prefix length (nxt_r - i)
            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len {
                break;
            }

            // Binary search in the sorted HashSet for this prefix length
            if sets[prefix_len].contains(&sh) {
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