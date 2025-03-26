use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn has_special_substring(s: String, k: i32) -> bool {
        let n = s.len();
        let mut cnt = 0;
        let chars: Vec<char> = s.chars().collect();
        
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line as the string `s`
    let s = lines.next().unwrap().expect("Failed to read input string");
    
    // Read the second line as the integer `k`
    let k: i32 = lines.next().unwrap().expect("Failed to read input k")
        .trim().parse().expect("k should be an integer");
    
    let sol = Solution;
    println!("{}", sol.has_special_substring(s, k));
}