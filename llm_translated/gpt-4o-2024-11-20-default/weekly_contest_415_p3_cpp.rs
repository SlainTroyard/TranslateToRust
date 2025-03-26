use std::collections::HashSet;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();

        const MOD: i64 = 1_070_777_777;

        // Using a random BASE similar to `uniform_int_distribution`
        let mut rng = rand::thread_rng();
        let base: i64 = rand::Rng::gen_range(&mut rng, 800_000_000..900_000_000);

        // Precomputing powers of BASE and prefix hashes
        let mut pow_base = vec![1; n + 1];
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pow_base[i + 1] = pow_base[i] * base % MOD;
            pre_hash[i + 1] = (pre_hash[i] * base + target.as_bytes()[i] as i64) % MOD;
        }

        // Function to compute hash of a substring [l, r)
        let sub_hash = |l: usize, r: usize| -> i64 {
            ((pre_hash[r] - pre_hash[l] * pow_base[r - l] % MOD + MOD) % MOD)
        };

        // Process words and find the maximum length
        let mut max_len = 0;
        for w in &words {
            max_len = max_len.max(w.len());
        }
        
        let mut sets = vec![HashSet::new(); max_len];
        for w in &words {
            let mut h: i64 = 0;
            for (j, &b) in w.as_bytes().iter().enumerate() {
                h = (h * base + b as i64) % MOD;
                sets[j].insert(h);
            }
        }

        // Determine the maximum "jump" for a given position `i`
        let max_jump = |i: usize| -> usize {
            let mut left = 0;
            let mut right = n.saturating_sub(i).min(max_len) + 1;
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

        // Solve the problem iteratively
        let mut ans = 0;
        let mut cur_r = 0;
        let mut nxt_r = 0;

        for i in 0..n {
            nxt_r = nxt_r.max(i + max_jump(i));
            if i == cur_r {
                if i == nxt_r {
                    return -1; // If unable to find a valid substring
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
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let mut lines = input.lines();
    let words_size: usize = lines.next().unwrap().parse().unwrap();
    
    let mut words = Vec::new();
    for _ in 0..words_size {
        let word_line = lines.next().unwrap();
        let mut parts = word_line.split_whitespace();
        let word_len: usize = parts.next().unwrap().parse().unwrap();
        let word = parts.next().unwrap().to_string();
        words.push(word);
    }

    let target_line = lines.next().unwrap();
    let mut parts = target_line.split_whitespace();
    let target_len: usize = parts.next().unwrap().parse().unwrap();
    let target = parts.next().unwrap().to_string();

    let solution = Solution;
    let result = solution.min_valid_strings(words, target);

    println!("{}", result);
}