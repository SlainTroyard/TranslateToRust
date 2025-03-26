use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let n = s.len();
        let mut mp: HashMap<String, i32> = HashMap::new();
        let size = n / k as usize;
        
        // Count occurrences of substrings in s
        for i in (0..n).step_by(size) {
            let substring = &s[i..i + size];
            *mp.entry(substring.to_string()).or_insert(0) += 1;
        }
        
        // Decrement counts for substrings in t
        for i in (0..n).step_by(size) {
            let substring = &t[i..i + size];
            *mp.entry(substring.to_string()).or_insert(0) -= 1;
        }
        
        // Check if all counts are zero
        for (_, value) in mp {
            if value != 0 {
                return false;
            }
        }
        
        true
    }
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();
    let mut k_str = String::new();
    
    // Read input strings and k
    io::stdin().read_line(&mut s).expect("Failed to read s");
    io::stdin().read_line(&mut t).expect("Failed to read t");
    io::stdin().read_line(&mut k_str).expect("Failed to read k");
    
    // Trim whitespace and newlines
    let s = s.trim().to_string();
    let t = t.trim().to_string();
    let k: i32 = k_str.trim().parse().expect("Failed to parse k as integer");
    
    // Call the solution function
    let result = Solution::is_possible_to_rearrange(s, t, k);
    
    // Output the result in the same format as the original code
    if result {
        println!("true");
    } else {
        println!("false");
    }
}