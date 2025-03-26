use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn max_difference(s: String) -> i32 {
        // Initialize a counter array for each letter (a-z)
        let mut cnt = [0; 26];
        
        // Count occurrences of each character
        for b in s.chars() {
            cnt[(b as u8 - b'a') as usize] += 1;
        }

        // Find maximum odd count and minimum even non-zero count
        let mut max1 = 0;
        let mut min0 = i32::MAX;
        
        for &c in &cnt {
            if c % 2 == 1 {
                // Odd count
                max1 = max1.max(c);
            } else if c > 0 {
                // Even non-zero count
                min0 = min0.min(c);
            }
        }
        
        // Return the difference
        max1 - min0
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the input string
    let s = lines.next().unwrap().unwrap();
    
    // Create solution instance and solve
    let sol = Solution {};
    println!("{}", Solution::max_difference(s));
}