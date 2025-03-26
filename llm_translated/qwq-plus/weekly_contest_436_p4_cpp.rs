use std::io;

struct Solution {}

impl Solution {
    pub fn max_score(&self, points: &Vec<i32>, m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m as i64;
            let mut pre = 0i64;
            for i in 0..n {
                let p = points[i] as i64;
                let mut k = (low - 1) / p + 1 - pre;
                if i == n - 1 && k <= 0 {
                    break;
                }
                k = k.max(1);
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                pre = k - 1;
            }
            true
        };

        let mut left: i64 = 0;
        let min_point = *points.iter().min().unwrap() as i64;
        let mut right = ((m as i64 + 1) / 2) * min_point + 1;

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
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: i32 = parts.next().unwrap().parse().unwrap();

    let points_line = lines.next().unwrap().unwrap();
    let points: Vec<i32> = points_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution {};
    let res = solution.max_score(&points, m);
    println!("{}", res);
}