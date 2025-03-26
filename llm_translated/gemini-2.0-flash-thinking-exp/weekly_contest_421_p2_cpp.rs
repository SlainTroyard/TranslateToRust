use std::io;

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn length_after_transformations(s: String, t: i32) -> i64 {
        let mut cnt = [0i64; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        for _ in 0..t {
            let mut nxt = [0i64; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            for j in 0..26 {
                cnt[j] = nxt[j];
            }
        }

        let mut ans = 0;
        for i in 0..26 {
            ans = (ans + cnt[i]) % MOD;
        }
        ans
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let parts: Vec<&str> = input_line.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let t: i32 = parts[1].parse().expect("Invalid integer");

    let solution = Solution;
    println!("{}", solution.length_after_transformations(s, t));
}