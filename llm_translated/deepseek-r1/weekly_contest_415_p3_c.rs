use std::io::{self, BufRead};
use rand::Rng;

const MOD: u64 = 1_070_777_777;

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    if n == 0 {
        return 0;
    }

    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    if max_len == 0 {
        return -1;
    }

    let mut rng = rand::thread_rng();
    let base = rng.gen_range(800_000_000u32..900_000_000u32) as u64;

    let mut pow_base = vec![1u64; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    let target_bytes = target.as_bytes();
    let mut pre_hash = vec![0u64; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target_bytes[i] as u64) % MOD;
    }

    let mut sets = vec![vec![]; max_len];
    for word in words {
        let word_bytes = word.as_bytes();
        let mut h = 0u64;
        for j in 0..word_bytes.len().min(max_len) {
            h = (h * base + word_bytes[j] as u64) % MOD;
            sets[j].push(h as u32);
        }
    }

    for set in &mut sets {
        set.sort_unstable();
        set.dedup();
    }

    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    for i in 0..n {
        while nxt_r < n && (nxt_r - i) < max_len {
            let end = nxt_r + 1;
            let len = end - i;
            let hash = (pre_hash[end] + MOD - (pre_hash[i] * pow_base[len] % MOD)) % MOD;
            let hash = hash as u32;

            let prefix_len = nxt_r - i;
            if prefix_len >= max_len {
                break;
            }

            if sets[prefix_len].binary_search(&hash).is_ok() {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let words_size: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        words.push(parts[1].to_string());
    }

    let line = lines.next().unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();
    let target = parts[1].to_string();

    println!("{}", min_valid_strings(&words, &target));
}