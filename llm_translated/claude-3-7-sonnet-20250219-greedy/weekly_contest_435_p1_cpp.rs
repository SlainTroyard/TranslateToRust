use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_difference(s: String) -> i32 {
        // Count occurrences of each character
        let mut cnt = [0; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        // Find maximum odd count and minimum non-zero even count
        let mut max1 = 0;
        let mut min0 = i32::MAX;
        
        for c in cnt.iter() {
            if c % 2 == 1 {
                // Odd count
                max1 = max1.max(*c);
            } else if *c > 0 {
                // Even non-zero count
                min0 = min0.min(*c);
            }
        }
        
        max1 - min0
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string
    if let Some(Ok(s)) = lines.next() {
        let sol = Solution;
        println!("{}", Solution::max_difference(s));
    }
}