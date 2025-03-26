use std::io;

struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        let s_bytes = s.as_bytes();

        for &c in s_bytes {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;

            while cnt[idx] >= k {
                let left_idx = (s_bytes[left] - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }
            ans += left as i32;
        }
        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let k: i32 = k_str.trim().parse().unwrap();

    let sol = Solution {};
    println!("{}", sol.number_of_substrings(s, k));
}