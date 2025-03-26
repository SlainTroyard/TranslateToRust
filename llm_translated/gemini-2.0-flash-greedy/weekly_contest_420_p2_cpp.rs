use std::io;

struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        let s_bytes = s.as_bytes();

        for &c in s_bytes {
            let index = (c - b'a') as usize;
            cnt[index] += 1;

            while cnt[index] >= k {
                let left_char = s_bytes[left];
                let left_index = (left_char - b'a') as usize;
                cnt[left_index] -= 1;
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