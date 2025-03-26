use std::io;
use std::cmp::{min, max};

struct Solution;

impl Solution {
    fn max_substring_length(s: String, k: i32) -> bool {
        let n = s.len();
        
        // Create vectors to store positions of each character
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
        for (i, c) in s.chars().enumerate() {
            let c_idx = (c as u8 - b'a') as usize;
            pos[c_idx].push(i);
        }
        
        // Find substrings that contain all occurrences of certain characters
        let mut vec: Vec<(usize, usize)> = Vec::new();
        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = pos[i][0];
                let mut r = *pos[i].last().unwrap();
                let mut flag = true;
                
                while flag {
                    flag = false;
                    for j in 0..26 {
                        // Count characters in the current range [l, r]
                        let lower = pos[j].partition_point(|&p| p < l);
                        let upper = pos[j].partition_point(|&p| p <= r);
                        let cnt = upper - lower;
                        
                        // If the range contains some (but not all) occurrences of character j
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
                
                // If this substring doesn't cover the whole string
                if l > 0 || r < n - 1 {
                    vec.push((r, l));
                }
            }
        }
        
        // Sort by right position and greedily select non-overlapping intervals
        vec.sort_unstable();
        let mut r: i32 = -1;
        let mut cnt = 0;
        
        for &(right, left) in &vec {
            if left as i32 > r {
                r = right as i32;
                cnt += 1;
            }
        }
        
        cnt >= k
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim().to_string();
    
    // Read K value
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read K");
    let k: i32 = k_input.trim().parse().expect("Invalid K value");
    
    // Solve the problem and print the result
    let sol = Solution::max_substring_length(s, k);
    println!("{}", if sol { 1 } else { 0 });
}