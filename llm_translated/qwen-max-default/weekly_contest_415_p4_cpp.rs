use std::io::{self, BufRead, Write};
use std::collections::HashSet;
use rand::Rng;
use std::time::SystemTime;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut words_size = String::new();
    stdin.lock().read_line(&mut words_size).unwrap();
    let words_size: usize = words_size.trim().parse().unwrap();

    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        words.push(parts[1].to_string());
    }

    let mut target_len = String::new();
    stdin.lock().read_line(&mut target_len).unwrap();
    let target_len: usize = target_len.trim().parse().unwrap();
    let mut target = String::new();
    stdin.lock().read_line(&mut target).unwrap();
    target = target.trim().to_string();

    let sol = Solution {};
    writeln!(stdout, "{}", sol.min_valid_strings(&words, &target)).unwrap();
}

struct Solution {}

impl Solution {
    fn min_valid_strings(&self, words: &[String], target: &str) -> i32 {
        let n = target.len();

        const MOD: i64 = 1_070_777_777;
        let rng = rand::thread_rng();
        let base: i64 = rng.gen_range(800_000_000..=900_000_000);
        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] * base + target.as_bytes()[i] as i64) % MOD + MOD) % MOD;
        }
        let sub_hash = |l: usize, r: usize| {
            ((pre_hash[r] - pre_hash[l] * pow_base[r - l]) % MOD + MOD) % MOD
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets = vec![HashSet::new(); max_len];
        for w in words {
            let mut h = 0;
            for (j, &ch) in w.as_bytes().iter().enumerate() {
                h = (h * base + ch as i64) % MOD;
                if j < max_len {
                    sets[j].insert(h);
                }
            }
        }

        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            let mut right = n - i + 1;
            if right > max_len {
                right = max_len + 1;
            }
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