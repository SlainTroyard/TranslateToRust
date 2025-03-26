use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        
        let n = s.len();
        let mut ans = String::new();
        
        for i in 0..n {
            // In Rust, we need to ensure our calculations are all in the same type (usize)
            let k_usize = k as usize;
            let end_pos = n - max(k_usize - 1, i);
            
            // Get the substring from position i with appropriate length
            let candidate = &s[i..i + end_pos];
            
            // Update ans if the current candidate is lexicographically greater
            if candidate > ans.as_str() {
                ans = candidate.to_string();
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read string s
    let s = lines.next().unwrap().unwrap();
    
    // Read integer k
    let k = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Call answer_string function
    let result = Solution::answer_string(s, k);
    
    // Print the result
    println!("{}", result);
}