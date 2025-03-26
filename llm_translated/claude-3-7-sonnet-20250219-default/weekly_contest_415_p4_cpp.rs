use std::collections::HashSet;
use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution {}

impl Solution {
    fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        
        const MOD: i32 = 1_070_777_777;
        
        // Initialize random number generator with current time
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
            
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let base = rand::Rng::gen_range(&mut rng, 800_000_000..900_000_000);
        
        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        pow_base[0] = 1;
        
        let target_bytes = target.as_bytes();
        for i in 0..n {
            pow_base[i + 1] = ((pow_base[i] as i64 * base as i64) % MOD as i64) as i32;
            pre_hash[i + 1] = ((pre_hash[i] as i64 * base as i64 + target_bytes[i] as i64) % MOD as i64) as i32;
        }
        
        let sub_hash = |l: usize, r: usize| -> i32 {
            ((pre_hash[r] as i64 - (pre_hash[l] as i64 * pow_base[r - l] as i64) % MOD as i64) % MOD as i64 + MOD as i64) % MOD as i64 as i32
        };
        
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); max_len];
        
        for w in &words {
            let mut h: i64 = 0;
            let w_bytes = w.as_bytes();
            for j in 0..w.len() {
                h = (h * base as i64 + w_bytes[j] as i64) % MOD as i64;
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
    // Set up stdin reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read words size
    let words_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read words
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        // Read word length (but we don't actually need it since strings in Rust already know their length)
        let _word_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        // Read word
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }
    
    // Read target length (also not needed)
    let _target_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read target
    let target = lines.next().unwrap().unwrap();
    
    // Solve and output result
    let sol = Solution {};
    println!("{}", sol.min_valid_strings(words, target));
}