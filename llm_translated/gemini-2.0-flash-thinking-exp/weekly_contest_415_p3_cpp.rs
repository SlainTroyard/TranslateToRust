use std::collections::HashSet;
use rand::Rng;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    fn min_valid_strings(&self, words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }

        const MOD: i64 = 1_070_777_777;
        let mut rng = rand::thread_rng();
        let base: i64 = rng.gen_range(800_000_000..900_000_000);
        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] * base) + (target.as_bytes()[i] as i64)) % MOD;
        }

        let sub_hash = |l: usize, r: usize| -> i64 {
            let val = (pre_hash[r] - (pre_hash[l] * pow_base[r - l]) % MOD) % MOD;
            if val < 0 {
                val + MOD
            } else {
                val
            }
        };

        let mut max_len = 0;
        for w in &words {
            max_len = max_len.max(w.len());
        }
        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        for w in &words {
            let mut h: i64 = 0;
            for j in 0..w.len() {
                h = (h * base + (w.as_bytes()[j] as i64)) % MOD;
                sets[j].insert(h);
            }
        }

        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            let mut right = (n - i).min(max_len) + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                if sets[mid - 1].contains(&sub_hash(i, i + mid)) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            left
        };

        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;
        for i in 0..n {
            nxt_r = nxt_r.max(i + max_jump(i));
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
}

fn main() {
    let mut words_size_str = String::new();
    io::stdin().read_line(&mut words_size_str).unwrap();
    let words_size: i32 = words_size_str.trim().parse().unwrap();
    let mut words: Vec<String> = Vec::new();
    for _ in 0..words_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let word_len: i32 = parts[0].parse().unwrap(); // word_len is read but not used
        words.push(parts[1].to_string());
    }
    let mut target_len_str = String::new();
    io::stdin().read_line(&mut target_len_str).unwrap();
    let target_len: i32 = target_len_str.trim().parse().unwrap(); // target_len is read but not used
    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    let sol = Solution {};
    println!("{}", sol.min_valid_strings(words, target));
}