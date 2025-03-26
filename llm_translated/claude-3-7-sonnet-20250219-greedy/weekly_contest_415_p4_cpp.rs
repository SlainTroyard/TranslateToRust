use std::collections::HashSet;
use std::io::{self, BufRead};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution;

impl Solution {
    fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let target_bytes = target.as_bytes();

        const MOD: i64 = 1_070_777_777;
        
        // Create a seeded random number generator
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        
        // Generate a random base between 8e8 and 9e8
        let base: i64 = rng.gen_range(800_000_000..900_000_000);
        
        let mut pow_base = vec![0i64; n + 1];
        let mut pre_hash = vec![0i64; n + 1];
        
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base) % MOD;
            pre_hash[i + 1] = ((pre_hash[i] * base) + target_bytes[i] as i64) % MOD;
        }
        
        // Closure to calculate substring hash
        let sub_hash = |l: usize, r: usize| -> i64 {
            ((pre_hash[r] - (pre_hash[l] * pow_base[r - l]) % MOD) % MOD + MOD) % MOD
        };

        // Find maximum word length
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        
        // Create sets of hashes for each length
        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        
        for w in &words {
            let w_bytes = w.as_bytes();
            let mut h: i64 = 0;
            for j in 0..w.len() {
                h = (h * base + w_bytes[j] as i64) % MOD;
                sets[j].insert(h);
            }
        }

        // Closure to find maximum jump from position i
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
    
    // Read words size
    let words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        // Read word length (but we don't actually need it since we can get it from the word itself)
        let _word_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        // Read the actual word
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }
    
    // Read target length (but we don't actually need it)
    let _target_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read target
    let target = lines.next().unwrap().unwrap();
    
    // Solve and output
    let sol = Solution::min_valid_strings(words, target);
    println!("{}", sol);
}