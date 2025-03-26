use std::io;

struct Solution;

impl Solution {
    fn length_after_transformations(&self, s: String, t: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut cnt = [0u64; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
        }
        for _ in 0..t {
            let mut next = [0u64; 26];
            for j in 0..25 {
                next[j + 1] = cnt[j];
            }
            next[0] = (next[0] + cnt[25]) % MOD;
            next[1] = (next[1] + cnt[25]) % MOD;
            cnt = next;
        }
        let total: u64 = cnt.iter().sum();
        (total % MOD) as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Input must have exactly two parts");
    }
    let s = parts[0].to_string();
    let t: i32 = parts[1].parse().expect("Invalid integer");
    let solution = Solution;
    let ans = solution.length_after_transformations(s, t);
    println!("{}", ans);
}