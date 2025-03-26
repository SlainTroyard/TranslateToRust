use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?.trim().to_string();
    let t: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Solve the problem
    let result = Solution::length_after_transformations(s, t);

    // Print result
    println!("{}", result);

    Ok(())
}

struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut cnt = [0i64; 26];

        // Count initial frequencies
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        // Apply transformations
        for _ in 1..=t {
            let mut nxt = [0i64; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            cnt = nxt;
        }

        // Calculate final result
        let ans: i64 = cnt.iter().sum::<i64>() % MOD;
        ans as i32
    }
}