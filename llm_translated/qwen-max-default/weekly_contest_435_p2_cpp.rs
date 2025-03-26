use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_distance(s: &str, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => unreachable!(),
            }
            ans = ans.max((x.abs() + y.abs() + k * 2).min(i as i32 + 1));
        }
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input
    let s = lines.next().unwrap().expect("Failed to read input string");
    let k: i32 = lines.next().unwrap().expect("Failed to read input integer").parse().expect("Failed to parse input integer");

    // Create an instance of the Solution and compute the result
    let sol = Solution;
    let result = sol.max_distance(&s, k);

    // Output the result to stdout
    println!("{}", result);
}