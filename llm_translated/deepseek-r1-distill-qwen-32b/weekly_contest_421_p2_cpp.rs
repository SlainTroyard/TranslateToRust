use std::io;

struct Solution {}

impl Solution {
    fn length_after_transformations(&self, s: &str, t: u64) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut cnt = [0u64; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
        }
        for _ in 0..t {
            let mut nxt = [0u64; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            cnt = nxt;
        }
        let mut ans = 0;
        for &c in &cnt {
            ans = (ans + c) % MOD;
        }
        ans as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let t: u64 = parts[1].parse().expect("Invalid t");
    let solution = Solution {};
    let result = solution.length_after_transformations(s, t);
    println!("{}", result);
}