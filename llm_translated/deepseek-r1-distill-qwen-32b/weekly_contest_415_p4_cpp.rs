use std::collections::HashSet;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

struct Solution;

impl Solution {
    fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }

        const MOD: i64 = 1_070_777_777;
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let base = rng.gen_range(800_000_000..=900_000_000);

        let mut pow_base = vec![0i64; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
        }

        let mut pre_hash = vec![0i64; n + 1];
        for i in 0..n {
            pre_hash[i + 1] = (pre_hash[i] * base + target.chars().nth(i).unwrap() as i64) % MOD;
        }

        let sub_hash = |l: usize, r: usize| -> i64 {
            let len = r - l;
            let hash = (pre_hash[r] - pre_hash[l] * pow_base[len]) % MOD;
            (hash + MOD) % MOD
        };

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<i64>> = Vec::with_capacity(max_len);
        for _ in 0..max_len {
            sets.push(HashSet::new());
        }

        for w in &words {
            let mut h = 0;
            for j in 0..w.len() {
                h = (h * base + w.chars().nth(j).unwrap() as i64) % MOD;
                if j < sets.len() {
                    sets[j].insert(h);
                }
            }
        }

        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            let mut right = std::cmp::min(n - i, max_len);
            while left + 1 < right {
                let mid = (left + right) / 2;
                let current_hash = sub_hash(i, i + mid);
                if mid - 1 < sets.len() && sets[mid - 1].contains(&current_hash) {
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
            let jump = max_jump(i);
            nxt_r = std::cmp::max(nxt_r, i + jump);
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut ptr = 0;
    let words_size = tokens[ptr].parse::<usize>().unwrap();
    ptr += 1;

    let mut words = Vec::new();
    for _ in 0..words_size {
        let word_len = tokens[ptr].parse::<usize>().unwrap();
        ptr += 1;
        let word = tokens[ptr].to_string();
        ptr += 1;
        words.push(word);
    }

    let target_len = tokens[ptr].parse::<usize>().unwrap();
    ptr += 1;
    let target = tokens[ptr].to_string();
    ptr += 1;

    let sol = Solution;
    let ans = sol.min_valid_strings(words, target);
    println!("{}", ans);
}