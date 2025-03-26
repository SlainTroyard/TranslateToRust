use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        
        let n = s.len();
        let mut ans = String::new();
        
        for i in 0..n {
            // Calculate the substring length based on the original logic
            let substring_length = n - max(k as usize - 1, i);
            
            // Extract the substring
            let current_substring = s.chars().skip(i).take(substring_length).collect::<String>();
            
            // Compare and update the answer
            if current_substring > ans {
                ans = current_substring;
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input string
    let s = lines.next().unwrap().unwrap();
    
    // Input k
    let k = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Call answer_string function
    let result = Solution::answer_string(s, k);
    
    // Print the result
    println!("{}", result);
}