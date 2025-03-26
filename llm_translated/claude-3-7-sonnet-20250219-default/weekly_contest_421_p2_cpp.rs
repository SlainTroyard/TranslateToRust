use std::io;

struct Solution {}

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
            
            // Apply the transformation rule
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            
            // Update counts
            for j in 0..26 {
                cnt[j] = nxt[j];
            }
        }
        
        // Calculate total length
        let mut ans = 0i64;
        for i in 0..26 {
            ans = (ans + cnt[i]) % MOD;
        }
        
        ans as i32
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");
    let s = input.trim().to_string();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read number");
    let t: i32 = input.trim().parse().expect("Input must be a number");
    
    // Solve and print result
    let solution = Solution {};
    println!("{}", solution::length_after_transformations(s, t));
}