use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut cnt = [0i64; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        for _ in 1..=t {
            let mut nxt = [0i64; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            cnt = nxt;
        }
        let mut ans = 0i64;
        for &count in cnt.iter() {
            ans = (ans + count) % MOD;
        }
        ans as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid integer input");
    
    let solution = Solution;
    let result = solution.length_after_transformations(s, t);
    println!("{}", result);
}