use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    fn is_possible_to_rearrange(s: String, t: String, k: usize) -> bool {
        let n = s.len();
        let mut mp: HashMap<String, i32> = HashMap::new();
        let size = n / k;
        
        // Count occurrences of each substring in s
        for i in (0..n).step_by(size) {
            let substring = &s[i..i+size];
            *mp.entry(substring.to_string()).or_insert(0) += 1;
        }
        
        // Decrease count for each substring in t
        for i in (0..n).step_by(size) {
            let substring = &t[i..i+size];
            *mp.entry(substring.to_string()).or_insert(0) -= 1;
        }
        
        // If all counts are zero, return true
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
    
    // Read input from stdin
    io::stdin().read_line(&mut s).expect("Failed to read string s");
    io::stdin().read_line(&mut t).expect("Failed to read string t");
    io::stdin().read_line(&mut k_str).expect("Failed to read k");
    
    // Trim whitespace and parse
    let s = s.trim().to_string();
    let t = t.trim().to_string();
    let k: usize = k_str.trim().parse().expect("Failed to parse k");
    
    // Call the solution function
    let result = Solution::is_possible_to_rearrange(s, t, k);
    
    // Print result in the expected format
    if result {
        println!("true");
    } else {
        println!("false");
    }
}