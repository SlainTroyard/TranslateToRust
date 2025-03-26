use std::io;
use std::io::prelude::*;
use std::cmp::{max, min};

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

        let mut left: i64 = 0;
        let min_element = *points.iter().min().unwrap() as i64;
        let mut right: i64 = (m as i64 + 1) / 2 * min_element + 1;
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
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let line1 = iter.next().unwrap().unwrap();
    let mut nums1 = line1.split_whitespace();
    let n: i32 = nums1.next().unwrap().parse().unwrap();
    let m: i32 = nums1.next().unwrap().parse().unwrap();

    let line2 = iter.next().unwrap().unwrap();
    let points: Vec<i32> = line2.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let solution = Solution {};
    println!("{}", solution.max_score(points, m));
}