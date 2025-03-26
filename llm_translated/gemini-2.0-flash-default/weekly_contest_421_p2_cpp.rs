use std::io;

struct Solution {}

impl Solution {
    pub fn length_after_transformations(&self, s: String, t: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt: [i64; 26] = [0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        for _ in 1..=t {
            let mut nxt: [i64; 26] = [0; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            for j in 0..26 {
                cnt[j] = nxt[j];
            }
        }

        let mut ans: i64 = 0;
        for i in 0..26 {
            ans = (ans + cnt[i]) % MOD;
        }

        ans as i32
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    let mut t_str = String::new();
    io::stdin().read_line(&mut t_str).unwrap();
    let t: i32 = t_str.trim().parse().unwrap();

    let solution = Solution {};
    println!("{}", solution.length_after_transformations(s, t));
}