use std::collections::HashSet;
use std::io;
use rand::Rng;

const MOD: i32 = 1_070_777_777;

struct Solution;

impl Solution {
    fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }
        let target_bytes = target.as_bytes();
        let base: i32 = rand::thread_rng().gen_range(800_000_000..=900_000_000);

        // Precompute pow_base and pre_hash arrays
        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pow_base[i + 1] = ((pow_base[i] as i64 * base as i64) % MOD as i64) as i32;
            pre_hash[i + 1] = ((pre_hash[i] as i64 * base as i64 + target_bytes[i] as i64) % MOD as i64) as i32;
        }

        // Helper function to compute hash of substring [l, r)
        let sub_hash = |l: usize, r: usize| -> i32 {
            let part1 = pre_hash[r] as i64;
            let part2 = (pre_hash[l] as i64) * (pow_base[r - l] as i64);
            let res = (part1 - part2) % (MOD as i64);
            let res = (res + MOD as i64) % (MOD as i64);
            res as i32
        };

        // Compute maximum word length
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        if max_len == 0 {
            return -1;
        }

        // Initialize sets
        let mut sets = vec![HashSet::new(); max_len];
        for word in &words {
            let mut h = 0i64;
            for (j, c) in word.bytes().enumerate() {
                h = (h * base as i64 + c as i64) % MOD as i64;
                sets[j].insert(h as i32);
            }
        }

        // Closure to find maximum jump length from position i
        let max_jump = |i: usize| -> usize {
            let max_possible = std::cmp::min(n - i, max_len);
            let mut left = 0;
            let mut right = max_possible + 1;
            while left + 1 < right {
                let mid = (left + right) / 2;
                let len = mid;
                let end = i + len;
                let hash_needed = sub_hash(i, end);
                let set_index = len - 1;
                if set_index < sets.len() && sets[set_index].contains(&hash_needed) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            left
        };

        // Greedy algorithm to find minimum jumps
        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;
        for i in 0..n {
            let current_max_jump = max_jump(i);
            let possible = i + current_max_jump;
            if possible > nxt_r {
                nxt_r = possible;
            }
            if i == cur_r {
                if nxt_r == i {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let words_size: usize = tokens.next().unwrap().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let _word_len: usize = tokens.next().unwrap().parse().unwrap();
        let word = tokens.next().unwrap().to_string();
        words.push(word);
    }
    let _target_len: usize = tokens.next().unwrap().parse().unwrap();
    let target = tokens.next().unwrap().to_string();

    let sol = Solution;
    let result = sol.min_valid_strings(words, target);
    println!("{}", result);
}