use std::io::{self, Write};

// Returns the maximum of two numbers
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Returns the minimum of two numbers
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// Main function implementation
fn max_difference(s: &str, k: usize) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();
    
    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }
            
            let mut cur_s = [0; 5]; // Current window digit counts
            let mut pre_s = [0; 5]; // Left part of the window digit counts
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum differences
            let mut left = 0;
            
            for (i, c) in s.chars().enumerate() {
                cur_s[c as usize - '0' as usize] += 1; // Update current window digit counts
                let r = i + 1;
                
                // When window size >= k and other conditions are met, try to shrink the left boundary
                while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] as usize & 1][pre_s[y] as usize & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); // Update minimum difference
                    pre_s[s[left..].chars().next().unwrap() as usize - '0' as usize] += 1; // Update left part of the window digit counts
                    left += 1;
                }
                
                // Update the answer
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1) as usize ^ 1][(cur_s[y] & 1) as usize]);
            }
        }
    }
    
    ans
}

fn main() {
    // Read input
    let mut s = String::new();
    let mut k = String::new();
    
    println!("Enter the string and k:");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut k).expect("Failed to read line");
    
    let s = s.trim();
    let k: usize = k.trim().parse().expect("Please enter a valid number for k");
    
    // Call the function to compute the result
    let result = max_difference(s, k);
    
    // Output the result
    println!("{}", result);
}