use std::cmp::{max, min};
use std::io;

struct Solution {}

impl Solution {
    pub fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            for i in 0..n {
                let k = (low - 1) / points[i] as i64 + 1 - pre;
                if i == n - 1 && k <= 0 {
                    break;
                }
                let k = max(k, 1);
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        let mut left = 0;
        let min_element = *points.iter().min().unwrap() as i64;
        let mut right = (m as i64 + 1) / 2 * min_element + 1;
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let points: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_score(points, m));
}