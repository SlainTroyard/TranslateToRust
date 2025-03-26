use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        
        for (i, ch) in s.chars().enumerate() {
            match ch {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().expect("Invalid integer for k");
    
    let sol = Solution;
    let result = sol.max_distance(s, k);
    println!("{}", result);
}