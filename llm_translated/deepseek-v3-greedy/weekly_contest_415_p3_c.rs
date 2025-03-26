use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};

const MOD: u64 = 1_070_777_777;

fn get_random_base() -> u64 {
    800_000_000 + (rand::random::<u64>() % 100_000_000)
}

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

    let base = get_random_base();

    let mut pow_base = vec![0; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    let mut pre_hash = vec![0; n + 1];
    for (i, &c) in target.as_bytes().iter().enumerate() {
        pre_hash[i + 1] = (pre_hash[i] * base + c as u64) % MOD;
    }

    let mut sets = vec![HashSet::new(); max_len];
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0;
        for (j, &c) in word.as_bytes().iter().take(len).enumerate() {
            h = (h * base + c as u64) % MOD;
            sets[j].insert(h);
        }
    }

    let mut sorted_sets: Vec<Vec<u64>> = sets
        .into_iter()
        .map(|set| {
            let mut vec: Vec<u64> = set.into_iter().collect();
            vec.sort();
            vec
        })
        .collect();

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
            let set = &sorted_sets[prefix_len];
            if set.binary_search(&current_hash).is_ok() {
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
    let mut lines = stdin.lock().lines();

    let words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let line = lines.next().unwrap().unwrap();
        let word = line.trim().to_string();
        words.push(word);
    }

    let target = lines.next().unwrap().unwrap().trim().to_string();

    let res = min_valid_strings(&words, &target);
    println!("{}", res);
}