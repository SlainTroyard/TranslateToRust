use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        let s_chars: Vec<char> = s.chars().collect();

        for &c in &s_chars {
            cnt[(c as u8 - b'a') as usize] += 1;
            while cnt[(c as u8 - b'a') as usize] >= k {
                cnt[(s_chars[left] as u8 - b'a') as usize] -= 1;
                left += 1;
            }
            ans += left as i32;
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string `s` from the first line
    let s = lines.next().unwrap().unwrap();
    
    // Read the integer `k` from the second line
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    let sol = Solution;
    println!("{}", sol.number_of_substrings(s, k));
}