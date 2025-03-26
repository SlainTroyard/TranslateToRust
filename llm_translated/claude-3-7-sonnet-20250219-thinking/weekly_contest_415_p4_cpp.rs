use std::collections::HashSet;
use std::io::{self, BufRead};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();

        const MOD: i32 = 1_070_777_777;
        
        // Generate random base similar to C++ version
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(now);
        let base = rng.gen_range(800_000_000..900_000_000) as i64;
        
        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        
        pow_base[0] = 1;
        let target_bytes = target.as_bytes();
        
        for i in 0..n {
            pow_base[i + 1] = ((pow_base[i] as i64 * base) % MOD as i64) as i32;
            pre_hash[i + 1] = ((pre_hash[i] as i64 * base + target_bytes[i] as i64) % MOD as i64) as i32;
        }
        
        let sub_hash = |l: usize, r: usize| -> i32 {
            let val = (pre_hash[r] as i64 - (pre_hash[l] as i64 * pow_base[r - l] as i64) % MOD as i64) % MOD as i64;
            ((val + MOD as i64) % MOD as i64) as i32
        };

        // Find maximum length of words
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
        
        // Create sets of word hashes by length
        let mut sets = vec![HashSet::new(); max_len];
        for w in &words {
            let w_bytes = w.as_bytes();
            let mut h: i64 = 0;
            for j in 0..w.len() {
                h = (h * base + w_bytes[j] as i64) % MOD as i64;
                sets[j].insert(h as i32);
            }
        }

        // Binary search function to find maximum jump length
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

        // Greedy algorithm to find minimum number of valid strings
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read words size
    let words_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    
    // Read each word
    for _ in 0..words_size {
        let word_len: usize = lines.next().unwrap()?.parse().unwrap();
        let word = lines.next().unwrap()?;
        words.push(word);
    }
    
    // Read target
    let target_len: usize = lines.next().unwrap()?.parse().unwrap();
    let target = lines.next().unwrap()?;
    
    let solution = Solution::min_valid_strings(words, target);
    println!("{}", solution);
    
    Ok(())
}