use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        
        for (i, c) in s.chars().enumerate() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => (),
            }
            ans = ans.max((x.abs() + y.abs() + k * 2).min((i + 1) as i32));
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line as the string `s`
    let s = lines.next().unwrap().expect("Failed to read input string");
    
    // Read the second line as the integer `k`
    let k: i32 = lines.next().unwrap().expect("Failed to read input integer")
        .trim().parse().expect("Failed to parse integer");
    
    let sol = Solution;
    println!("{}", sol.max_distance(s, k));
}