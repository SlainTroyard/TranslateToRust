use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: usize) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt = vec![0i64; 26];
        
        // Count occurrences of each character in the input string
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        
        // Perform transformations for `t` iterations
        for _ in 0..t {
            let mut nxt = vec![0i64; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            cnt = nxt;
        }
        
        // Calculate the final answer
        let mut ans = 0i64;
        for &count in &cnt {
            ans = (ans + count) % MOD;
        }
        
        ans as i32
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // First line: the string `s`
    let s = lines.next().unwrap().unwrap();
    
    // Second line: the integer `t`
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.length_after_transformations(s, t);
    
    // Print the result to stdout
    println!("{}", result);
}