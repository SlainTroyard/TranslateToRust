use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt = [0i64; 26];
        
        // Count occurrences of each character
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        
        // Apply transformations t times
        for _ in 1..=t {
            let mut nxt = [0i64; 26];
            
            // Apply the transformation rules
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            
            // Update counts for next iteration
            for j in 0..26 {
                cnt[j] = nxt[j];
            }
        }
        
        // Calculate the total length after transformations
        let mut ans = 0i64;
        for i in 0..26 {
            ans = (ans + cnt[i]) % MOD;
        }
        
        ans as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input string and transformation count
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Calculate and print the result
    let solution = Solution;
    println!("{}", Solution::length_after_transformations(s, t));
}