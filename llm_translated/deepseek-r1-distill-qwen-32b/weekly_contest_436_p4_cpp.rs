use std::io;

struct Solution;

impl Solution {
    fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let n = points.len();
        if n == 0 || m == 0 {
            return 0;
        }

        let min_point = *points.iter().min().unwrap() as i64;
        let mut left = 0;
        let mut right = ((m as i64 + 1) / 2) * min_point + 1;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if Self::check(mid, &points, m as i64) {
                left = mid;
            } else {
                right = mid;
            }
        }

        left
    }

    fn check(low: i64, points: &[i32], m: i64) -> bool {
        let n = points.len();
        let mut rem = m;
        let mut pre = 0;

        for i in 0..n {
            let point = points[i] as i64;
            let k = ((low - 1) / point + 1) - pre;
            let k = if k <= 0 && i == n - 1 {
                0
            } else {
                std::cmp::max(k, 1)
            };

            rem -= (k * 2) - 1;
            if rem < 0 {
                return false;
            }

            pre = k - 1;
        }

        true
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: i32 = parts.next().unwrap().parse().unwrap();

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = parts.next().unwrap().parse().unwrap();
        points.push(num);
    }

    let solution = Solution;
    let result = solution.max_score(points, m);
    println!("{}", result);

    Ok(())
}