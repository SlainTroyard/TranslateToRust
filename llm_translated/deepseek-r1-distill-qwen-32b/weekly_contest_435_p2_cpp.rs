use std::io;

struct Solution;

impl Solution {
    fn max_distance(s: String, k: i32) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut ans = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => {}
            }
            let current_step = i as i32 + 1;
            let distance = (x.abs() + y.abs() + k * 2).min(current_step);
            ans = ans.max(distance);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k = parts[1]
        .parse::<i32>()
        .expect("Failed to parse k as integer");
    
    let result = Solution::max_distance(s, k);
    println!("{}", result);
}