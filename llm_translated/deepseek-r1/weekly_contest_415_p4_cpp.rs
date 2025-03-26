use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        const MOD: i64 = 1_070_777_777;
        let n = target.len();
        if n == 0 {
            return 0;
        }

        // Generate random base between 800_000_000 and 900_000_000
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut rng = StdRng::seed_from_u64(seed);
        let base = rng.gen_range(800_000_000..=900_000_000) as i64;

        // Precompute pow_base and pre_hash for target
        let target_bytes = target.as_bytes();
        let mut pow_base = vec![1i64; n + 1];
        let mut pre_hash = vec![0i64; n + 1];
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = (pre_hash[i] * base + target_bytes[i] as i64) % MOD;
        }

        // Closure to compute substring hash
        let sub_hash = |l: usize, r: usize| -> i64 {
            let len = r - l;
            let part = (pre_hash[l] * pow_base[len]) % MOD;
            let hash = (pre_hash[r] - part) % MOD;
            (hash + MOD) % MOD
        };

        // Build hash sets for each possible prefix length
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets = vec![HashSet::new(); max_len];
        for word in &words {
            let mut h = 0i64;
            for (j, &c) in word.as_bytes().iter().enumerate() {
                h = (h * base + c as i64) % MOD;
                if j < max_len {
                    sets[j].insert(h);
                }
            }
        }

        // Binary search to find maximum jump length from position i
        let max_jump = |i: usize| -> usize {
            let remaining = n - i;
            let max_possible = remaining.min(max_len);
            let mut left = 0;
            let mut right = max_possible + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                if mid > 0 {
                    let hash = sub_hash(i, i + mid);
                    if sets[mid - 1].contains(&hash) {
                        left = mid;
                    } else {
                        right = mid;
                    }
                } else {
                    right = mid;
                }
            }
            left
        };

        // Greedy algorithm to find minimal number of valid strings
        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;
        for i in 0..n {
            let jump = max_jump(i);
            nxt_r = nxt_r.max(i + jump);
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
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut tokens = lines.join(" ").split_whitespace().collect::<Vec<_>>();
    let mut ptr = 0;

    // Read input following the exact format of the original C++ code
    let words_size: usize = tokens[ptr].parse().unwrap();
    ptr += 1;

    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let _word_len: usize = tokens[ptr].parse().unwrap();
        ptr += 1;
        let word = tokens[ptr].to_string();
        ptr += 1;
        words.push(word);
    }

    let _target_len: usize = tokens[ptr].parse().unwrap();
    ptr += 1;
    let target = tokens[ptr].to_string();

    println!("{}", Solution::min_valid_strings(words, target));
}