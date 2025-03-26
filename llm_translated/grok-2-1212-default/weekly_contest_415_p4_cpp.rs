use std::collections::HashSet;
use std::io::{self, BufRead};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: &[String], target: &str) -> i32 {
        let n = target.len();
        const MOD: i64 = 1_070_777_777;

        let mut rng = ChaCha8Rng::from_entropy();
        let base: i64 = rng.gen_range(800_000_000..900_000_000);
        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];

        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] as i64 * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] as i64 * base) + target.as_bytes()[i] as i64) % MOD;
        }

        let sub_hash = |l: usize, r: usize| -> i64 {
            (((pre_hash[r] - (pre_hash[l] as i64 * pow_base[r - l]) % MOD) + MOD) % MOD)
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];

        for w in words {
            let mut h: i64 = 0;
            for (j, &c) in w.as_bytes().iter().enumerate() {
                h = (h * base + c as i64) % MOD;
                sets[j].insert(h);
            }
        }

        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            let mut right = std::cmp::min(n - i, max_len) + 1;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let words_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut words = Vec::with_capacity(words_size);

    for _ in 0..words_size {
        let _ = lines.next().unwrap()?.parse::<usize>().unwrap(); // word_len
        words.push(lines.next().unwrap()?);
    }

    let _ = lines.next().unwrap()?.parse::<usize>().unwrap(); // target_len
    let target = lines.next().unwrap()?;

    let sol = Solution;
    let result = sol.min_valid_strings(&words, &target);
    println!("{}", result);

    Ok(())
}