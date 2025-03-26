use std::io;

struct Solution;

impl Solution {
    fn max_distance(s: String, k: i32) -> i32 {
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
            let distance = x.abs() + y.abs();
            let candidate = (distance + k * 2).min((i + 1) as i32);
            ans = ans.max(candidate);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        panic!("Input must contain a string and an integer");
    }
    let s = parts[0].to_string();
    let k = parts[1].parse().expect("Second token must be an integer");
    let result = Solution::max_distance(s, k);
    println!("{}", result);
}