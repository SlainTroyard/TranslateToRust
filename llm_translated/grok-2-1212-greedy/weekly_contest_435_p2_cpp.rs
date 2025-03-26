use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(s: &str, k: i32) -> i32 {
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

            let distance = x.abs() + y.abs() + k * 2;
            ans = ans.max(distance.min((i + 1) as i32));
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?.trim().to_string();
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Calculate and print result
    let sol = Solution;
    println!("{}", sol.max_distance(&s, k));

    Ok(())
}