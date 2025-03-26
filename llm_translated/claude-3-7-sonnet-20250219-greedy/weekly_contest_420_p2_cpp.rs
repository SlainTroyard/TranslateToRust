use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        
        for (right, &c) in s_bytes.iter().enumerate() {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
            
            // While the count of the current character is >= k, move the left pointer
            while cnt[idx] >= k {
                let left_idx = (s_bytes[left] - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }
            
            // Add the current left position to the answer
            ans += left as i32;
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the input string
    let s = lines.next().unwrap().unwrap();
    
    // Read the value of k
    let k = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Calculate and print the result
    let result = Solution::number_of_substrings(s, k);
    println!("{}", result);
}