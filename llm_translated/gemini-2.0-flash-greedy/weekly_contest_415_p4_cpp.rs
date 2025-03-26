use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashSet;
use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();

        const MOD: i64 = 1_070_777_777;
        //let mut rng = rand::thread_rng(); // Use thread_rng for actual randomness, but StdRng for testing
        let mut rng: StdRng = SeedableRng::from_seed([0u8; 32]); //use deterministic seed to match cpp output format in example
        let base: i64 = rng.gen_range(800_000_000..900_000_000);

        let mut pow_base = vec![0; n + 1];
        let mut pre_hash = vec![0; n + 1];
        pow_base[0] = 1;
        for i in 0..n {
            pow_base[i + 1] = (pow_base[i] * base % MOD) as i64;
            pre_hash[i + 1] = ((pre_hash[i] * base % MOD) + target.as_bytes()[i] as i64) % MOD;
        }

        let sub_hash = |l: usize, r: usize| -> i64 {
            (((pre_hash[r] - pre_hash[l] * pow_base[r - l]) % MOD + MOD) % MOD) as i64
        };

        let mut max_len = 0;
        for w in &words {
            max_len = max_len.max(w.len());
        }

        let mut sets: Vec<HashSet<i64>> = vec![HashSet::new(); max_len];
        for w in &words {
            let mut h: i64 = 0;
            for j in 0..w.len() {
                h = (h * base + w.as_bytes()[j] as i64) % MOD;
                sets[j].insert(h);
            }
        }

        let max_jump = |i: usize| -> usize {
            let mut left: usize = 0;
            let mut right: usize = (n - i).min(max_len) + 1;
            while left + 1 < right {
                let mid: usize = (left + right) / 2;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let words_size: usize = lines.next().unwrap().parse()?;
    let mut words: Vec<String> = Vec::with_capacity(words_size);

    for _ in 0..words_size {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let _word_len: usize = split.next().unwrap().parse()?;
        words.push(split.next().unwrap().to_string());
    }

    let target_len: usize = lines.next().unwrap().parse()?;
    let target: String = lines.next().unwrap().to_string();

    let sol = Solution {};
    println!("{}", sol.min_valid_strings(words, target));

    Ok(())
}