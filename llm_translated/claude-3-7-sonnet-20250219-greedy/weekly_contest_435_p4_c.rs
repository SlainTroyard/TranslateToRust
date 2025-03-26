use std::io::{self, BufRead};
use std::cmp::{min, max};

// Main function implementation
fn max_difference(s: &str, k: i32) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let s_bytes = s.as_bytes();
    let len = s.len();
    
    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }
            
            let mut cur_s = [0; 5]; // Current window digit counts
            let mut pre_s = [0; 5]; // Left part of window digit counts
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum differences
            let mut left = 0;
            
            for i in 0..len {
                let digit = (s_bytes[i] - b'0') as usize;
                cur_s[digit] += 1; // Update current window digit counts
                let r = i + 1;
                
                // Try to shrink the left boundary when conditions are met
                while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p_idx = [(pre_s[x] & 1) as usize, (pre_s[y] & 1) as usize];
                    min_s[p_idx[0]][p_idx[1]] = min(min_s[p_idx[0]][p_idx[1]], pre_s[x] - pre_s[y]);
                    let left_digit = (s_bytes[left] - b'0') as usize;
                    pre_s[left_digit] += 1; // Update left part counts
                    left += 1;
                }
                
                // Update answer
                let idx = [((cur_s[x] & 1) ^ 1) as usize, (cur_s[y] & 1) as usize];
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[idx[0]][idx[1]]);
            }
        }
    }
    
    ans
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse input line
    let input_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = input_line.split_whitespace().collect();
    
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }
    
    let s = parts[0];
    let k = parts[1].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Error parsing k");
        std::process::exit(1);
    });
    
    // Call function to calculate result
    let result = max_difference(s, k);
    
    // Output result
    println!("{}", result);
}