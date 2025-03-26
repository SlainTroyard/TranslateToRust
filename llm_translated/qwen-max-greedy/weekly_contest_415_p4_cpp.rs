use std::io::{self, BufRead, Write};
use std::collections::HashSet;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of words
    stdin.lock().read_line(&mut buffer).unwrap();
    let words_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        stdin.lock().read_line(&mut buffer).unwrap();
        let word_len: usize = buffer.trim().parse().unwrap();
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let word = buffer.trim().to_string();
        assert_eq!(word.len(), word_len); // Ensure the word length matches
        words.push(word);
        buffer.clear();
    }

    // Read the target
    stdin.lock().read_line(&mut buffer).unwrap();
    let target_len: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    stdin.lock().read_line(&mut buffer).unwrap();
    let target = buffer.trim().to_string();
    assert_eq!(target.len(), target_len); // Ensure the target length matches

    // Create and solve
    let sol = Solution {};
    let result = sol.min_valid_strings(words, &target);
    writeln!(stdout, "{}", result).unwrap();
}

struct Solution;

impl Solution {
    pub fn min_valid_strings(&self, words: Vec<String>, target: &str) -> i32 {
        let n = target.len();
        const MOD: i64 = 1_070_777_777;
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let rng = rand::thread_rng();
        let base: i64 = rng.gen_range(800_000_000..=900_000_000);

        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] * base + target.as_bytes()[i] as i64) % MOD + MOD) % MOD;
        }

        let sub_hash = |l: usize, r: usize| -> i64 {
            ((pre_hash[r] - (pre_hash[l] * pow_base[r - l]) % MOD + MOD) % MOD) as i64
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        for w in &words {
            let mut h = 0;
            for (j, &c) in w.as_bytes().iter().enumerate() {
                h = (h * base + c as i64) % MOD;
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