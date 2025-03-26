use std::io::{self, Read};
use std::cmp::{max, min};

struct Solution;

impl Solution {
    fn max_difference(s: String) -> i32 {
        // Initialize an array to count occurrences of each lowercase letter
        let mut cnt = [0; 26];
        
        // Count occurrences of each letter
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        // Track max count of letters with odd occurrences
        // and min count of letters with even occurrences
        let mut max1 = 0;
        let mut min0 = i32::MAX;
        
        for &c in &cnt {
            if c % 2 == 1 {
                // Odd count
                max1 = max(max1, c);
            } else if c > 0 {
                // Even count (but not zero)
                min0 = min(min0, c);
            }
        }
        
        // Return the difference
        max1 - min0
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim().to_string();
    
    // Create solution instance and compute result
    let sol = Solution;
    println!("{}", sol.max_difference(s));
}