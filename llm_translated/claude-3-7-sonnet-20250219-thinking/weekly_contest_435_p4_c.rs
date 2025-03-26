use std::io;
use std::cmp;

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
            
            let mut cur_s = [0i32; 5]; // Current window digit counts
            let mut pre_s = [0i32; 5]; // Left part of window digit counts
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum differences
            let mut left = 0;
            
            for i in 0..len {
                let digit = (s_bytes[i] - b'0') as usize;
                cur_s[digit] += 1;
                let r = i + 1;
                
                // Try to shrink window from left when conditions are met
                while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let idx1 = (pre_s[x] & 1) as usize;
                    let idx2 = (pre_s[y] & 1) as usize;
                    min_s[idx1][idx2] = cmp::min(min_s[idx1][idx2], pre_s[x] - pre_s[y]);
                    let left_digit = (s_bytes[left] - b'0') as usize;
                    pre_s[left_digit] += 1;
                    left += 1;
                }
                
                // Update answer with current window's difference and minimum difference
                let idx1 = ((cur_s[x] & 1) ^ 1) as usize;
                let idx2 = (cur_s[y] & 1) as usize;
                ans = cmp::max(ans, cur_s[x] - cur_s[y] - min_s[idx1][idx2]);
            }
        }
    }
    
    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Failed to parse k");
    
    // Calculate result
    let result = max_difference(s, k);
    
    // Output result
    println!("{}", result);
}