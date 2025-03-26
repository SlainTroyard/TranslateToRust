use std::collections::HashSet;
use std::io;
use rand::Rng;

struct Solution;

impl Solution {
    fn min_valid_strings(&self, words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        if n == 0 {
            return 0;
        }

        const MOD: u64 = 1_070_777_777;
        let mut rng = rand::thread_rng();
        let base = rng.gen_range(800_000_000..=900_000_000);

        let mut pow_base = vec![0; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
        }

        let target_bytes: Vec<u64> = target.chars().map(|c| c as u64).collect();
        let mut pre_hash = vec![0; n + 1];
        for i in 0..n {
            pre_hash[i + 1] = (pre_hash[i] * base + target_bytes[i]) % MOD;
        }

        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<u64>> = Vec::with_capacity(max_len);
        for _ in 0..max_len {
            sets.push(HashSet::new());
        }

        for word in &words {
            let mut h = 0;
            for j in 0..word.len() {
                h = (h * base + (word.chars().nth(j).unwrap() as u64)) % MOD;
                if j < sets.len() {
                    sets[j].insert(h);
                }
            }
        }

        let max_jump = |i: usize| -> usize {
            let max_possible = std::cmp::min(n - i, max_len);
            let mut left = 0;
            let mut right = max_possible + 1;

            while left + 1 < right {
                let mid = (left + right) / 2;
                if mid == 0 {
                    right = mid;
                    continue;
                }
                if mid > max_possible {
                    right = mid;
                    continue;
                }
                let end = i + mid;
                if end > n {
                    right = mid;
                    continue;
                }
                let len = mid;
                let hash = (pre_hash[end] - pre_hash[i] * pow_base[len]) % MOD;
                let hash = (hash + MOD) % MOD;

                if let Some(set) = sets.get(mid - 1) {
                    if set.contains(&hash) {
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

        let mut cur_r = 0;
        let mut nxt_r = 0;
        let mut ans = 0;

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

        ans as i32
    }
}

fn main() {
    let mut words_size = String::new();
    io::stdin().read_line(&mut words_size).expect("Failed to read words_size");
    let words_size: usize = words_size.trim().parse().expect("Invalid words_size");

    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read word");
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let word_len: usize = parts[0].parse().expect("Invalid word_len");
        let word = parts[1].to_string();
        words.push(word);
    }

    let mut target_len_line = String::new();
    io::stdin().read_line(&mut target_len_line).expect("Failed to read target_len");
    let target_len: usize = target_len_line.trim().parse().expect("Invalid target_len");

    let mut target_line = String::new();
    io::stdin().read_line(&mut target_line).expect("Failed to read target");
    let target = target_line.trim().to_string();

    let sol = Solution;
    let result = sol.min_valid_strings(words, target);
    println!("{}", result);
}