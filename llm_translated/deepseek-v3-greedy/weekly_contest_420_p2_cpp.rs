use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        let s_bytes = s.as_bytes();

        for &c in s_bytes {
            cnt[(c - b'a') as usize] += 1;
            while cnt[(c - b'a') as usize] >= k {
                cnt[(s_bytes[left] - b'a') as usize] -= 1;
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
    
    // Read the first line as the string `s`
    let s = lines.next().unwrap().expect("Failed to read input string");
    
    // Read the second line as the integer `k`
    let k: i32 = lines.next().unwrap().expect("Failed to read input k")
        .trim().parse().expect("Failed to parse k as integer");
    
    let sol = Solution;
    println!("{}", sol.number_of_substrings(s, k));
}