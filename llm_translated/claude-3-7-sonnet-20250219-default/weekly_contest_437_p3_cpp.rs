use std::io::{self, BufRead};
use std::cmp::{min, max};
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        let n = s.len();
        let s_bytes = s.as_bytes();

        // Create position vectors for each character
        let mut pos: Vec<Vec<i32>> = vec![Vec::new(); 26];
        for i in 0..n {
            let c = (s_bytes[i] - b'a') as usize;
            pos[c].push(i as i32);
        }

        // Find the segments
        let mut vec: Vec<(i32, i32)> = Vec::new();
        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = pos[i][0];
                let mut r = *pos[i].last().unwrap();
                let mut flag = true;
                
                while flag {
                    flag = false;
                    for j in 0..26 {
                        // Count characters in the current range
                        let cnt = pos[j].iter()
                            .filter(|&&pos| pos >= l && pos <= r)
                            .count();
                        
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
                
                if l > 0 || r < (n as i32) - 1 {
                    vec.push((r, l));
                }
            }
        }

        // Sort segments and count non-overlapping ones
        vec.sort();
        let mut r = -1;
        let mut cnt = 0;
        
        for (end, start) in vec {
            if start > r {
                r = end;
                cnt += 1;
            }
        }
        
        cnt >= k
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input
    let s = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Solve and print output
    let sol = Solution {};
    let result = Solution::max_substring_length(s, k);
    
    println!("{}", if result { 1 } else { 0 });
}