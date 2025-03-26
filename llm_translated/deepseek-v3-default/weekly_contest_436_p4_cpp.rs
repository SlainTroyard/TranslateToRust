use std::io;
use std::cmp::{max, min};

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let m: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Read the points vector
    let mut points = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let point: i64 = input.trim().parse().expect("Invalid input");
        points.push(point);
    }

    // Create an instance of the Solution and compute the result
    let solution = Solution;
    let result = solution.max_score(&points, m);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn max_score(&self, points: &[i64], m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            for i in 0..n {
                let mut k = (low - 1) / points[i] + 1 - pre;
                if i == n - 1 && k <= 0 {
                    break;
                }
                k = max(k, 1);
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        let mut left = 0;
        let mut right = (m + 1) as i64 / 2 * *points.iter().min().unwrap() + 1;
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