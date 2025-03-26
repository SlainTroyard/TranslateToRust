use std::collections::HashSet;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

const MOD: i32 = 1070777777;

fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let n = target.len();
    if n == 0 {
        return 0;
    }

    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);
    if max_len == 0 {
        return 0;
    }

    let base = 800_000_000 + (rand::thread_rng().gen_range(0..100_000_000)) as u64;

    let mut pow_base = vec![1; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD as u64;
    }

    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD as u64;
    }

    let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); max_len];
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0;
        for j in 0..len {
            h = (h * base + word.as_bytes()[j] as u64) % MOD as u64;
            sets[j].insert(h as i32);
        }
    }

    for set in &mut sets {
        let mut vec: Vec<i32> = set.drain().collect();
        vec.sort();
        *set = vec.into_iter().collect();
    }

    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    while cur_r < n {
        while nxt_r < n && (nxt_r - cur_r) < max_len as usize {
            let prefix_len = nxt_r - cur_r;
            if prefix_len >= max_len {
                break;
            }

            let current_hash = (pre_hash[nxt_r + 1] + MOD as u64 - (pre_hash[cur_r] * pow_base[nxt_r + 1 - cur_r]) % MOD as u64) % MOD as u64;
            let sh = current_hash as i32;

            let set = &sets[prefix_len];
            if set.contains(&sh) {
                nxt_r += 1;
            } else {
                break;
            }
        }

        if cur_r == nxt_r {
            return -1;
        }

        ans += 1;
        cur_r = nxt_r;
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().peekable();
    let words_size: usize = lines.next().unwrap().parse().unwrap();

    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let word_len: usize = lines.next().unwrap().parse().unwrap();
        let word = lines.next().unwrap().to_string();
        words.push(word);
    }

    let target_size: usize = lines.next().unwrap().parse().unwrap();
    let target = lines.next().unwrap().to_string();

    let result = min_valid_strings(words, target);
    println!("{}", result);
}