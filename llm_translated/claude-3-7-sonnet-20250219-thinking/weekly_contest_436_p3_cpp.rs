use std::io;

struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans: i64 = 0;
        let mut f: [[i32; 9]; 10] = [[0; 9]; 10];
        
        for c in s.chars() {
            let d = (c as u8 - b'0') as usize;
            
            for m in 1..10 {
                let mut nf = [0; 9];
                nf[d % m] = 1;
                
                for rem in 0..m {
                    nf[(rem * 10 + d) % m] += f[m][rem];
                }
                
                f[m] = nf;
            }
            
            ans += f[d][0] as i64;
        }
        
        ans
    }
}

fn main() {
    // Read input string from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();
    
    // Create solution instance and call the function
    let solution = Solution {};
    println!("{}", solution.count_substrings(s));
}