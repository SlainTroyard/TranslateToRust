use rand::Rng;
use std::collections::HashSet;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const MOD: i64 = 1_070_777_777;

struct Solution {}

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();

        let mut rng = rand::thread_rng();
        let base = rng.gen_range(800_000_000..900_000_000);

        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base as i64 % MOD) as i64;
            pre_hash[i + 1] = ((pre_hash[i] * base as i64) + target.as_bytes()[i] as i64) % MOD;
        }

        let sub_hash = |l: usize, r: usize| -> i64 {
            ((pre_hash[r] - (pre_hash[l] * pow_base[r - l]) % MOD) % MOD + MOD) % MOD
        };

        let mut max_len = 0;
        for w in &words {
            max_len = max_len.max(w.len());
        }

        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        for w in &words {
            let mut h: i64 = 0;
            for j in 0..w.len() {
                h = (h * base as i64 + w.as_bytes()[j] as i64) % MOD;
                sets[j].insert(h);
            }
        }

        let max_jump = |i: usize| -> usize {
            let mut left: usize = 0;
            let mut right: usize = (n - i).min(max_len) + 1;
            while left + 1 < right {
                let mid: usize = (left + right) / 2;
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
    let mut words_size = String::new();
    io::stdin().read_line(&mut words_size).unwrap();
    let words_size: i32 = words_size.trim().parse().unwrap();

    let mut words: Vec<String> = Vec::new();
    for _ in 0..words_size {
        let mut word_len = String::new();
        io::stdin().read_line(&mut word_len).unwrap();
        let word_len: i32 = word_len.trim().parse().unwrap();

        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        let word = word.trim().to_string();

        words.push(word);
    }

    let mut target_len = String::new();
    io::stdin().read_line(&mut target_len).unwrap();
    let target_len: i32 = target_len.trim().parse().unwrap();

    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    let sol = Solution {};
    println!("{}", sol.min_valid_strings(words, target));
}