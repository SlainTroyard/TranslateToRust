use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        const INF: i32 = i32::MAX / 2;
        let mut ans = -INF;
        
        // Convert string to vector of digits
        let s_chars: Vec<usize> = s.chars().map(|c| (c as u8 - b'0') as usize).collect();
        
        for x in 0..5 {
            for y in 0..5 {
                if y == x {
                    continue;
                }
                
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[INF, INF], [INF, INF]];
                let mut left = 0;
                
                for i in 0..s_chars.len() {
                    cur_s[s_chars[i]] += 1;
                    let r = i + 1;
                    
                    while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let parity_x = pre_s[x] & 1;
                        let parity_y = pre_s[y] & 1;
                        
                        min_s[parity_x][parity_y] = min_s[parity_x][parity_y].min(pre_s[x] - pre_s[y]);
                        pre_s[s_chars[left]] += 1;
                        left += 1;
                    }
                    
                    let parity_x_comp = (cur_s[x] & 1) ^ 1;  // Complement of parity
                    let parity_y = cur_s[y] & 1;
                    
                    ans = ans.max(cur_s[x] - cur_s[y] - min_s[parity_x_comp][parity_y]);
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
    
    let s = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().expect("Failed to parse k");
    
    // Solve
    let solution = Solution::max_difference(s, k);
    
    // Output
    println!("{}", solution);
}