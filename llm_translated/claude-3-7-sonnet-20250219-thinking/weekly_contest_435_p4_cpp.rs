use std::io;
use std::cmp::{max, min};

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
                
                let mut cur_s = [0i32; 5];
                let mut pre_s = [0i32; 5];
                let mut min_s = [[inf, inf], [inf, inf]];
                let mut left = 0;
                
                for i in 0..s.len() {
                    // Increment count for current digit
                    cur_s[(s.as_bytes()[i] - b'0') as usize] += 1;
                    let r = i + 1;
                    
                    // Slide window to maintain constraints
                    while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                        *p = min(*p, pre_s[x] - pre_s[y]);
                        pre_s[(s.as_bytes()[left] - b'0') as usize] += 1;
                        left += 1;
                    }
                    
                    // Update the answer
                    ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1 ^ 1) as usize][(cur_s[y] & 1) as usize]);
                }
            }
        }
        
        ans
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read string");
    let s = s.trim().to_string();
    
    // Read input integer k
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read k");
    let k: i32 = k_input.trim().parse().expect("Failed to parse k");
    
    // Solve the problem
    let sol = Solution::max_difference(s, k);
    
    // Output the result
    println!("{}", sol);
}