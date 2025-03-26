use std::io;

struct Solution;

impl Solution {
    fn has_special_substring(s: &str, k: usize) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut cnt = 0;
        
        for i in 0..n {
            cnt += 1;
            // Check if we're at the end or current char is different from next
            if i == n - 1 || chars[i] != chars[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0;
            }
        }
        
        false
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Parse input - same format as original C++ code
    let s = parts[0].to_string();
    let k: usize = parts[1].parse().expect("Invalid integer input");
    
    // Create solution and print result
    let sol = Solution;
    println!("{}", if sol.has_special_substring(&s, k) { 1 } else { 0 });
}