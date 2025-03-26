use std::io;

struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans: i64 = 0;
        // f[m][rem] stores the count of subsequences with remainder rem when divided by m
        let mut f = [[0; 9]; 10];
        
        for c in s.chars() {
            let d = (c as u8 - b'0') as usize;
            
            for m in 1..10 {
                let mut nf = [0; 9];
                // A single digit d with remainder d % m
                nf[d % m] = 1;
                
                // For each previous remainder, calculate new remainder after adding digit d
                for rem in 0..m {
                    nf[(rem * 10 + d) % m] += f[m][rem];
                }
                
                f[m] = nf;
            }
            
            // Add to result the number of substrings ending at current position
            // with remainder 0 when divided by d
            if d > 0 {  // Skip if d is 0 as we can't divide by 0
                ans += f[d][0] as i64;
            }
        }
        
        ans
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");
    let s = s.trim().to_string();
    
    // Create solution instance and compute result
    let sol = Solution {};
    let result = Solution::count_substrings(s);
    
    // Output result
    println!("{}", result);
}