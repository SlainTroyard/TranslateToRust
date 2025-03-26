use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, Write};
use std::ptr::null;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD: u64 = 1_070_777_777;

fn get_random_base() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let seed = since_the_epoch.as_secs() as u32;
    let mut rng = rand::thread_rng();
    800_000_000 + (rand::Rng::gen_range(&mut rng, 0..100_000_000))
}

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);
    let base = get_random_base() as u64;

    let mut pow_base = vec![0u64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    let mut pre_hash = vec![0u64; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD;
    }

    let mut sets = vec![HashSet::new(); max_len];
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0u64;
        for j in 0..len {
            h = (h * base + word.as_bytes()[j] as u64) % MOD;
            sets[j].insert(h as i32);
        }
    }

    let mut sorted_sets: Vec<Vec<i32>> = sets
        .into_iter()
        .map(|set| {
            let mut vec: Vec<i32> = set.into_iter().collect();
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
            let current_hash =
                (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            let sh = current_hash as i32;

            let prefix_len = nxt_r - i;
            if prefix_len < 0 || prefix_len >= max_len {
                break;
            }

            let set = &sorted_sets[prefix_len];
            if set.binary_search(&sh).is_ok() {
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