use std::collections::HashSet;
use std::io::{self, Write};

const MOD: u64 = 1_070_777_777;

fn get_random_base() -> u64 {
    // Use a simple random number generator for demonstration purposes
    // In a real application, consider using a proper random number generator
    800_000_000 + (rand::random::<u64>() % 100_000_000)
}

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    let base = get_random_base();

    let mut pow_base = vec![0; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    let mut pre_hash = vec![0; n + 1];
    for (i, &ch) in target.as_bytes().iter().enumerate() {
        pre_hash[i + 1] = (pre_hash[i] * base + ch as u64) % MOD;
    }

    let mut sets: Vec<HashSet<u64>> = vec![HashSet::new(); max_len];

    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0;
        for (j, &ch) in word.as_bytes().iter().take(len).enumerate() {
            h = (h * base + ch as u64) % MOD;
            sets[j].insert(h);
        }
    }

    for set in &mut sets {
        let mut sorted: Vec<u64> = set.iter().cloned().collect();
        sorted.sort_unstable();
        *set = sorted.into_iter().collect();
    }

    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        while nxt_r < n && (nxt_r - i) < max_len {
            if nxt_r + 1 > n {
                break;
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let prefix_len = nxt_r - i;
            if prefix_len >= max_len {
                break;
            }
            if sets[prefix_len].contains(&current_hash) {
                nxt_r += 1;
            } else {
                break;
            }
        }
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
    io::stdin().read_line(&mut input).unwrap();
    let words_size: usize = input.trim().parse().unwrap();

    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let word = input.trim().to_string();
        words.push(word);
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let target = input.trim().to_string();

    let res = min_valid_strings(&words, &target);
    println!("{}", res);
}