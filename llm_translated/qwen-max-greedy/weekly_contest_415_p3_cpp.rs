use std::io::{self, BufRead, Write};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let words_size: usize = input.trim().parse().expect("Please type a number!");
    
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let word_len: usize = input.trim().parse().expect("Please type a number!");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        words.push(input.trim().to_string());
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target_len: usize = input.trim().parse().expect("Please type a number!");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target = input.trim().to_string();

    let sol = Solution {};
    let result = sol.min_valid_strings(&words, &target);
    writeln!(io::stdout(), "{}", result).expect("Failed to write to stdout");
}

struct Solution;

impl Solution {
    pub fn min_valid_strings(&self, words: &[String], target: &str) -> i32 {
        let n = target.len();
        const MOD: u64 = 1_070_777_777;
        let seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng: ChaCha20Rng = SeedableRng::seed_from_u64(seed);
        let base: u64 = rng.gen_range(800_000_000..=900_000_000);

        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD);
        }
        let sub_hash = |l: usize, r: usize| -> u64 {
            ((pre_hash[r] - (pre_hash[l] * pow_base[r - l]) % MOD) % MOD + MOD) % MOD
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<std::collections::HashSet<u64>> = vec![std::collections::HashSet::new(); max_len];
        for w in words {
            let mut h: u64 = 0;
            for (j, &c) in w.as_bytes().iter().enumerate() {
                h = (h * base + c as u64) % MOD;
                if j < max_len {
                    sets[j].insert(h);
                }
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
            nxt_r = std::cmp::max(nxt_r, i + max_jump(i));
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