use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::io::{self, BufRead};

const MOD: i32 = 1_070_777_777;

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }

        // Generate random base between 800,000,000 and 900,000,000
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let base = rng.gen_range(800_000_000..=900_000_000) as i32;

        // Precompute pow_base and pre_hash for target
        let target_bytes = target.as_bytes();
        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pow_base[i + 1] = ((pow_base[i] as i64 * base as i64) % MOD as i64) as i32;
            pre_hash[i + 1] = ((pre_hash[i] as i64 * base as i64 + target_bytes[i] as i64) % MOD as i64) as i32;
        }

        // Closure to compute hash of substring [l, r)
        let sub_hash = |l: usize, r: usize| -> i32 {
            let len = r - l;
            let hash_l = pre_hash[l] as i64;
            let hash_r = pre_hash[r] as i64;
            let pow = pow_base[len] as i64;
            let mut res = (hash_r - hash_l * pow) % MOD as i64;
            if res < 0 {
                res += MOD as i64;
            }
            res as i32
        };

        // Build hash sets for each prefix length
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); max_len];
        for word in &words {
            let mut h: i64 = 0;
            for (j, &c) in word.as_bytes().iter().enumerate() {
                h = (h * base as i64 + c as i64) % MOD as i64;
                if j < max_len {
                    sets[j].insert(h as i32);
                }
            }
        }

        // Closure to find maximum valid jump from position i
        let max_jump = |i: usize| -> usize {
            let remaining = n - i;
            let max_possible = remaining.min(max_len);
            let mut left = 0;
            let mut right = max_possible + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                let hash = sub_hash(i, i + mid);
                if sets[mid - 1].contains(&hash) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            left
        };

        // Greedy algorithm to find minimal steps
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read words
    let words_size: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let line = lines.next().unwrap();
        let word = line.split_whitespace().nth(1).unwrap().to_string();
        words.push(word);
    }

    // Read target
    let line = lines.next().unwrap();
    let target = line.split_whitespace().nth(1).unwrap().to_string();

    println!("{}", Solution::min_valid_strings(words, target));
}