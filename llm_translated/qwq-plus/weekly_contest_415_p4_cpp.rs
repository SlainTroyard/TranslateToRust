use std::collections::HashSet;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }

        let mod_val = 1_070_777_777 as i64;

        // Generate random base between 8e8 and 9e8
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        let base = rng.gen_range(800_000_000..=900_000_000);

        // Precompute pow_base and pre_hash
        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base as i64) % mod_val;
            pre_hash[i + 1] = (pre_hash[i] * base as i64 + target.as_bytes()[i] as i64) % mod_val;
        }

        // Sub_hash function
        let sub_hash = |l: usize, r: usize| -> i64 {
            let len = r - l;
            let numerator = pre_hash[r] - pre_hash[l] * pow_base[len];
            let hash = (numerator % mod_val + mod_val) % mod_val;
            hash
        };

        // Compute max_len
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

        // Initialize sets
        let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); max_len];

        // Populate sets
        for word in words.iter() {
            let mut h: i64 = 0;
            for (j, &c) in word.as_bytes().iter().enumerate() {
                h = (h * base as i64 + c as i64) % mod_val;
                if j < sets.len() {
                    sets[j].insert(h as i32);
                }
            }
        }

        // Max_jump function
        let max_jump = |i: usize| -> usize {
            let max_possible = std::cmp::min(n - i, max_len);
            let mut left = 0;
            let mut right = max_possible + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                let l = i;
                let r = i + mid;
                let hash_val = sub_hash(l, r);
                let set_index = mid - 1;
                if set_index < 0 || set_index >= sets.len() {
                    right = mid;
                    continue;
                }
                let found = sets[set_index].contains(&(hash_val as i32));
                if found {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            left
        };

        // Main loop
        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;
        for i in 0..n {
            let current_jump = max_jump(i);
            let possible = i + current_jump;
            if possible > nxt_r {
                nxt_r = possible;
            }
            if i == cur_r {
                if i == nxt_r {
                    return -1;
                }
                cur_r = nxt_r;
                ans += 1;
            }
        }
        ans as i32
    }
}

fn main() {
    let mut words_size_str = String::new();
    std::io::stdin().read_line(&mut words_size_str).expect("Failed to read words_size");
    let words_size: usize = words_size_str.trim().parse().expect("Invalid words_size");

    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let mut word_len_str = String::new();
        std::io::stdin().read_line(&mut word_len_str).expect("Failed to read word_len");
        let _word_len: usize = word_len_str.trim().parse().expect("Invalid word_len");

        let mut word_str = String::new();
        std::io::stdin().read_line(&mut word_str).expect("Failed to read word");
        let word = word_str.trim().to_string();
        words.push(word);
    }

    let mut target_len_str = String::new();
    std::io::stdin().read_line(&mut target_len_str).expect("Failed to read target_len");
    let _target_len: usize = target_len_str.trim().parse().expect("Invalid target_len");

    let mut target_str = String::new();
    std::io::stdin().read_line(&mut target_str).expect("Failed to read target");
    let target = target_str.trim().to_string();

    let sol = Solution;
    let result = sol.min_valid_strings(words, target);
    println!("{}", result);
}