use std::io::{self, BufRead, Write};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    // Read the number of words
    let words_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);

    // Read each word
    for _ in 0..words_size {
        let word_len: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let word = lines.next().unwrap().unwrap();
        assert_eq!(word.len(), word_len);
        words.push(word);
    }

    // Read the target string
    let target_len: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let target = lines.next().unwrap().unwrap();
    assert_eq!(target.len(), target_len);

    // Create a solution instance and solve the problem
    let sol = Solution {};
    writeln!(stdout, "{}", sol.min_valid_strings(&words, &target)).unwrap();
}

struct Solution {}

impl Solution {
    pub fn min_valid_strings(&self, words: &[String], target: &str) -> i32 {
        let n = target.len();

        const MOD: u64 = 1_070_777_777;
        let rng = ChaCha8Rng::from_entropy();
        let base: u64 = rng.gen_range(800_000_000..=900_000_000);
        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];

        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] * base + target.as_bytes()[i] as u64) % MOD);
        }

        let sub_hash = |l: usize, r: usize| -> u64 {
            (((pre_hash[r] - pre_hash[l] * pow_base[r - l]) % MOD) + MOD) % MOD
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<u64>> = vec![HashSet::new(); max_len];

        for w in words {
            let mut h: u64 = 0;
            for (j, &c) in w.as_bytes().iter().enumerate() {
                h = (h * base + c as u64) % MOD;
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