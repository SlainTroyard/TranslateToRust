// Problem: Weekly Contest 436 Problem 4
use std::io;
use std::io::BufRead;
use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_score(&self, points: Vec<i32>, m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            for i in 0..n {
                let k = (low - 1) / (points[i] as i64) + 1 - pre;
                let mut current_k = k;
                if i == n - 1 && current_k <= 0 {
                    current_k = 0; // Ensure k is not negative in the last iteration
                } else {
                    current_k = max(current_k, 1);
                }

                rem -= current_k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = current_k - 1;
            }
            return true;
        };

        let mut left: i64 = 0;
        let min_point = *points.iter().min().unwrap();
        let mut right: i64 = (m as i64 + 1) / 2 * (min_point as i64) + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        return left;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().expect("Failed to read line");
    let mut nm_iter = first_line.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().expect("Failed to parse n");
    let m: i32 = nm_iter.next().unwrap().parse().expect("Failed to parse m");

    let second_line = iterator.next().unwrap().expect("Failed to read line");
    let points: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse point"))
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_score(points, m));
}