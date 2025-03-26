use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_score(points: &[i32], m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            
            for i in 0..n {
                let mut k = ((low - 1) / points[i] as i64) + 1 - pre;
                if i == n - 1 && k <= 0 {
                    break;
                }
                k = k.max(1);
                rem -= k as i32 * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        let mut left = 0i64;
        let min_element = *points.iter().min().unwrap_or(&0);
        let mut right = (m as i64 + 1) / 2 * (min_element as i64) + 1;
        
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
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: i32 = parts.next().unwrap().parse().unwrap();
    
    // Read points
    let line = lines.next().unwrap().unwrap();
    let points: Vec<i32> = line.split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution;
    println!("{}", solution.max_score(&points, m));
}