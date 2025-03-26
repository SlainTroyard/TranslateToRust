use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_score(points: &Vec<i32>, m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            for i in 0..n {
                let k = (low - 1) / points[i] as i64 + 1 - pre as i64;
                if i == n - 1 && k <= 0 {
                    break;
                }
                let k = k.max(1);
                rem -= (k * 2 - 1) as i32;
                if rem < 0 {
                    return false;
                }
                pre = k as i32 - 1;
            }
            true
        };

        let mut left = 0;
        let mut right = ((m as i64 + 1) / 2) * (*points.iter().min().unwrap() as i64) + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let nm: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    // Read points
    let points: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::max_score(&points, m);
    println!("{}", result);

    Ok(())
}