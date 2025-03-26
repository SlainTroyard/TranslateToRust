use std::collections::HashSet;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();

        const MOD: i64 = 1_070_777_777;
        let base = {
            let mut rng = rand::thread_rng();
            rand::Rng::gen_range(&mut rng, 800_000_000..900_000_000)
        };

        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] as i64 * base % MOD) as i32;
            pre_hash[i + 1] = ((pre_hash[i] as i64 * base + target.as_bytes()[i] as i64) % MOD) as i32;
        }

        let sub_hash = |l: usize, r: usize| -> i32 {
            (((pre_hash[r] as i64 - pre_hash[l] as i64 * pow_base[r - l] as i64) % MOD + MOD) % MOD) as i32
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); max_len];
        for w in &words {
            let mut h = 0i64;
            for (j, &b) in w.as_bytes().iter().enumerate() {
                h = (h * base + b as i64) % MOD;
                sets[j].insert(h as i32);
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of words
    let words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the words
    let mut words = Vec::new();
    for _ in 0..words_size {
        let word_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }

    // Read the target string
    let target_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let target = lines.next().unwrap().unwrap();

    // Solve the problem
    let solution = Solution;
    let result = solution.min_valid_strings(words, target);

    // Print the result
    println!("{}", result);
}