// Problem: Weekly Contest 437 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut cnt = 0;
        
        for i in 0..n {
            cnt += 1;
            if i == n - 1 || s_chars[i] != s_chars[i + 1] {
                if cnt == k as usize {
                    return true;
                }
                cnt = 0;
            }
        }
        
        false
    }
}

fn main() {
    // Read from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the input string
    let s = lines.next().unwrap().unwrap();
    
    // Read the value of k
    let k = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Solve the problem
    let sol = Solution;
    
    // Output the result in the same format as the original
    println!("{}", sol.has_special_substring(s, k) as i32);
}