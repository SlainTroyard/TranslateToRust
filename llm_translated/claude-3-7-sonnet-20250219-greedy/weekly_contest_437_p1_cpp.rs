use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn has_special_substring(s: &str, k: i32) -> bool {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut cnt = 0;
        
        for i in 0..n {
            cnt += 1;
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

fn main() -> io::Result<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the input string
    let s = lines.next().unwrap()?.trim().to_string();
    
    // Parse the integer k
    let k: i32 = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse k as an integer");
    
    // Create solution instance and call the function
    let sol = Solution;
    let result = Solution::has_special_substring(&s, k);
    
    // Output the result
    println!("{}", if result { 1 } else { 0 });
    
    Ok(())
}