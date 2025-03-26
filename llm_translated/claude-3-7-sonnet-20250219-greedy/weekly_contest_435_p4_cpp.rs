use std::io::{self, BufRead};
use std::cmp::{min, max};

struct Solution;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let inf = i32::MAX / 2;
        let mut ans = -inf;
        
        for x in 0..5 {
            for y in 0..5 {
                if y == x {
                    continue;
                }
                
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[inf, inf], [inf, inf]];
                let mut left = 0;
                
                for i in 0..s.len() {
                    let digit = (s.as_bytes()[i] - b'0') as usize;
                    cur_s[digit] += 1;
                    let r = i + 1;
                    
                    while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p_idx1 = (pre_s[x] & 1) as usize;
                        let p_idx2 = (pre_s[y] & 1) as usize;
                        min_s[p_idx1][p_idx2] = min(min_s[p_idx1][p_idx2], pre_s[x] - pre_s[y]);
                        
                        let left_digit = (s.as_bytes()[left] - b'0') as usize;
                        pre_s[left_digit] += 1;
                        left += 1;
                    }
                    
                    let idx1 = ((cur_s[x] & 1) ^ 1) as usize;
                    let idx2 = (cur_s[y] & 1) as usize;
                    ans = max(ans, cur_s[x] - cur_s[y] - min_s[idx1][idx2]);
                }
            }
        }
        
        ans
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the string s
    let s = lines.next().unwrap().unwrap();
    
    // Parse the integer k
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Solve the problem
    let result = Solution::max_difference(s, k);
    
    // Output the result
    println!("{}", result);
}