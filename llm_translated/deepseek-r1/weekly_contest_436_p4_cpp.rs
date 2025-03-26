use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn max_score(points: &[i32], m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m as i64;
            let mut pre = 0i64;

            for (i, &p) in points.iter().enumerate() {
                let p = p as i64;
                let k = (low - 1) / p + 1 - pre;

                if i == n - 1 && k <= 0 {
                    break;
                }

                let k = k.max(1);
                rem -= k * 2 - 1;

                if rem < 0 {
                    return false;
                }

                pre = k - 1;
            }

            true
        };

        let min_point = *points.iter().min().unwrap() as i64;
        let m_i64 = m as i64;
        let mut right = ((m_i64 + 1) / 2) * min_point + 1;
        let mut left = 0i64;

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
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: i32 = tokens.next().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        points.push(tokens.next().unwrap().parse().unwrap());
    }

    println!("{}", Solution::max_score(&points, m));
}