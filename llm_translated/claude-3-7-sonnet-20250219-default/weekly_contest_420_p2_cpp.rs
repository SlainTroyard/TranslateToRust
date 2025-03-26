use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn number_of_substrings(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        
        for (right, &c) in s_bytes.iter().enumerate() {
            // Convert ASCII character to index (0-25)
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
            
            // While the current character appears k or more times, move the left pointer
            while cnt[idx] >= k {
                let left_idx = (s_bytes[left] - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }
            
            // Add left to the answer (number of valid substrings ending at right)
            ans += left as i32;
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input string
    let s = lines.next().unwrap()?.trim().to_string();
    
    // Read k
    let k = lines.next().unwrap()?.trim().parse::<i32>().unwrap();
    
    // Calculate and print the result
    let result = Solution::number_of_substrings(s, k);
    println!("{}", result);
    
    Ok(())
}