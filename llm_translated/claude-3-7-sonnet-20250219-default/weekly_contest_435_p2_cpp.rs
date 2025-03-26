use std::io;
use std::cmp;

struct Solution;

impl Solution {
    fn max_distance(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        
        for (i, ch) in s.chars().enumerate() {
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => (), // Ignore other characters if any
            }
            
            // Calculate the maximum distance possible at this point
            let current_distance = cmp::min(x.abs() + y.abs() + k * 2, (i + 1) as i32);
            ans = cmp::max(ans, current_distance);
        }
        
        ans
    }
}

fn main() {
    // Read input
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read string");
    let s = input_line.trim().to_string();
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read k");
    let k: i32 = input_line.trim().parse().expect("Failed to parse k as i32");
    
    // Calculate and output result
    let sol = Solution;
    println!("{}", Solution::max_distance(s, k));
}