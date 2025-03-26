use std::io::{self, BufRead};
use std::cmp::{max, min};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();

    // Read the second line containing the points
    let second_line = lines.next().unwrap().unwrap();
    let points: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create a Solution instance and compute the max score
    let solution = Solution;
    let result = solution.max_score(&points, m);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn max_score(&self, points: &[i32], m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            for i in 0..n {
                let k = ((low - 1) / points[i] as i64 + 1 - pre).max(1);
                if i == n - 1 && k <= 0 {
                    break;
                }
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        let mut left = 0;
        let mut right = (m + 1) as i64 / 2 * *points.iter().min().unwrap() as i64 + 1;
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