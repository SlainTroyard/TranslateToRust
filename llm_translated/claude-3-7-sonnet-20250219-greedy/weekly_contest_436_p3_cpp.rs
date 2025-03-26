use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans: i64 = 0;
        // f[m][rem] stores the count of substrings with remainder rem when divided by m
        let mut f = [[0; 9]; 10];
        
        for c in s.chars() {
            let d = (c as u8 - b'0') as usize;
            
            for m in 1..10 {
                let mut nf = [0; 9];
                // A single digit d has remainder d % m
                nf[d % m] = 1;
                
                for rem in 0..m {
                    // For each existing remainder, calculate the new remainder when appending digit d
                    nf[(rem * 10 + d) % m] += f[m][rem];
                }
                
                f[m] = nf;
            }
            
            // Add the count of substrings ending at current position that are divisible by d
            if d > 0 { // Skip if d is 0 since we can't divide by 0
                ans += f[d][0] as i64;
            }
        }
        
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string
    let s = lines.next().unwrap().unwrap();
    
    // Create solution instance and call the function
    let sol = Solution;
    println!("{}", sol.count_substrings(s));
}