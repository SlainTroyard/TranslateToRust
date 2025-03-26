use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input strings and k value
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Calculate result
    let result = Solution::is_possible_to_rearrange(s, t, k);
    
    // Output result in the same format as the original
    if result {
        println!("true");
    } else {
        println!("false");
    }
}