use std::io::{self, BufRead};
use std::cmp::{min, max};

struct Solution;

impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        let n = s.len();
        let s_bytes = s.as_bytes();

        // Create position vectors for each character
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
        for i in 0..n {
            let c = (s_bytes[i] - b'a') as usize;
            pos[c].push(i);
        }

        // Find segments
        let mut vec: Vec<(usize, usize)> = Vec::new();
        for i in 0..26 {
            if !pos[i].is_empty() {
                let mut l = pos[i][0];
                let mut r = *pos[i].last().unwrap();
                let mut flag = true;
                
                while flag {
                    flag = false;
                    for j in 0..26 {
                        // Count occurrences of character j in the current range [l, r]
                        let cnt = pos[j].iter()
                            .filter(|&&pos| pos >= l && pos <= r)
                            .count();
                        
                        // If some but not all occurrences of character j are in the range
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
                
                // If the segment doesn't span the entire string
                if l > 0 || r < n - 1 {
                    vec.push((r, l));
                }
            }
        }

        // Sort segments by end position
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Greedy selection of non-overlapping segments
        let mut r = 0;
        let mut cnt = 0;
        for &(end, start) in &vec {
            if start >= r {
                r = end + 1; // Move to position after the current segment
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
    
    // Solve and output
    let sol = Solution;
    println!("{}", if sol.max_substring_length(s, k) { "1" } else { "0" });
}