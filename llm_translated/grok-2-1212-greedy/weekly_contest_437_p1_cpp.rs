use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn has_special_substring(s: &str, k: i32) -> bool {
        let n = s.len();
        let mut cnt = 0;
        for i in 0..n {
            cnt += 1;
            if i == n - 1 || s.chars().nth(i).unwrap() != s.chars().nth(i + 1).unwrap() {
                if cnt == k as usize {
                    return true;
                }
                cnt = 0;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?.trim().to_string();
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Process and output result
    let sol = Solution;
    println!("{}", sol.has_special_substring(&s, k));

    Ok(())
}